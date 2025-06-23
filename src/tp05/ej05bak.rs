#![allow(dead_code, unused_variables)]
use crate::tp03::ej03::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct StreamingRust {
    usuarios_activos: Vec<Usuario>,
    usuarios_cancelados: Vec<Usuario>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Usuario {
    nombre: String,
    subscripciones: Vec<Subscripcion>,
    email: String,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Subscripcion {
    costo_mensual: f32,
    duracion: u8,
    fecha_inicio: Fecha,
    metodo_pago: MetodoPago,
    tipo_subscripcion: TipoSubscripcion,
    usr_email: String,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Serialize, Deserialize)]
pub enum MetodoPago {
    Efectivo,
    MercadoPago { alias_mp: String },
    TarjetaCredito { num_tarjeta: String },
    Transferencia { cbu: String },
    Cripto { direccion: String, moneda: String },
}

#[derive(PartialEq, Clone, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum TipoSubscripcion {
    Basic,
    Classic,
    Super,
}

impl TipoSubscripcion {
    fn upgrade(&mut self) {
        *self = match self {
            Self::Basic => Self::Classic,
            Self::Classic => Self::Super,
            Self::Super => Self::Super,
        };
    }

    fn downgrade(&mut self) {
        *self = match self {
            Self::Basic => Self::Basic,
            Self::Classic => Self::Basic,
            Self::Super => Self::Classic,
        };
    }
}

impl Usuario {
    pub fn new(nombre: String, email: String) -> Self {
        Usuario {
            nombre,
            subscripciones: vec![],
            email,
        }
    }

    pub fn add_subscripcion(&mut self, subscripcion: Subscripcion) {
        self.subscripciones.push(subscripcion);
    }
}

impl Subscripcion {
    pub fn new(
        costo_mensual: f32,
        duracion: u8,
        metodo_pago: MetodoPago,
        tipo_subscripcion: TipoSubscripcion,
        usr_email: String,
    ) -> Self {
        Subscripcion {
            costo_mensual,
            duracion,
            fecha_inicio: Fecha::fecha_actual(),
            metodo_pago,
            tipo_subscripcion,
            usr_email,
        }
    }
}

impl StreamingRust {
    pub fn new() -> Self {
        let sistema = StreamingRust {
            usuarios_activos: vec![],
            usuarios_cancelados: vec![],
        };
        sistema.persistir();
        sistema
    }

    pub fn crear_usr(&mut self, usuario: &Usuario) {
        let mut sistema = Self::cargar();
        sistema.usuarios_activos.push(usuario.clone());
        sistema.persistir();
        *self = sistema;
    }

    fn buscar_usuario(&mut self, email: &str) -> Option<&mut Usuario> {
        let mut sistema = Self::cargar();
        let usuario = sistema
            .usuarios_activos
            .iter_mut()
            .find(|u| u.email == email);
        sistema.persistir();
        *self = sistema;
        self.usuarios_activos.iter_mut().find(|u| u.email == email)
    }

    fn buscar_usuario_i(&mut self, email: &str) -> Option<usize> {
        let mut sistema = Self::cargar();
        let index = sistema
            .usuarios_activos
            .iter()
            .position(|u| u.email == email);
        sistema.persistir();
        *self = sistema;
        self.usuarios_activos.iter().position(|u| u.email == email)
    }

    pub fn upgrade_subscripcion(&mut self, email: &str, sub_index: usize) -> bool {
        let mut sistema = Self::cargar();
        if let Some(user) = sistema
            .usuarios_activos
            .iter_mut()
            .find(|u| u.email == email)
        {
            if sub_index < user.subscripciones.len() {
                user.subscripciones[sub_index].tipo_subscripcion.upgrade();
                sistema.persistir();
                *self = sistema;
                return true;
            }
        }
        false
    }

    pub fn cancelar_subscripcion(&mut self, email: &str, sub_index: usize) -> bool {
        let mut sistema = Self::cargar();
        if let Some(user) = sistema
            .usuarios_activos
            .iter_mut()
            .find(|u| u.email == email)
        {
            if sub_index < user.subscripciones.len() {
                if user.subscripciones.len() == 1 {
                    if let Some(i) = sistema
                        .usuarios_activos
                        .iter()
                        .position(|u| u.email == email)
                    {
                        let usuario_cancelado = sistema.usuarios_activos.swap_remove(i);
                        sistema.usuarios_cancelados.push(usuario_cancelado);
                        sistema.persistir();
                        *self = sistema;
                        return true;
                    }
                } else {
                    user.subscripciones.remove(sub_index);
                    sistema.persistir();
                    *self = sistema;
                    return true;
                }
            }
        }
        false
    }

    pub fn downgrade_subscripcion(&mut self, email: &str, sub_index: usize) -> bool {
        let mut sistema = Self::cargar();
        if let Some(user) = sistema
            .usuarios_activos
            .iter_mut()
            .find(|u| u.email == email)
        {
            if sub_index < user.subscripciones.len() {
                if user.subscripciones[sub_index].tipo_subscripcion == TipoSubscripcion::Basic {
                    let result = sistema.cancelar_subscripcion(email, sub_index);
                    *self = sistema;
                    return result;
                } else {
                    user.subscripciones[sub_index].tipo_subscripcion.downgrade();
                    sistema.persistir();
                    *self = sistema;
                    return true;
                }
            }
        }
        false
    }

    fn metodo_mas_utilizado_generico(coleccion: &[Usuario]) -> Option<MetodoPago> {
        let mut cantidades: HashMap<MetodoPago, usize> = HashMap::new();
        let mut max_cant: usize = 0;
        let mut metodo_max = None;
        for user in coleccion.iter() {
            for sub in user.subscripciones.iter() {
                let metodo_act = sub.metodo_pago.clone();
                let count = cantidades.entry(metodo_act.clone()).or_insert(0);
                *count += 1;
                if *count > max_cant {
                    metodo_max = Some(metodo_act);
                    max_cant = *count;
                }
            }
        }
        metodo_max
    }

    pub fn metodo_mas_utilizado_activos(&self) -> Option<MetodoPago> {
        Self::metodo_mas_utilizado_generico(&self.usuarios_activos)
    }

    pub fn metodo_mas_utilizado(&self) -> Option<MetodoPago> {
        Self::metodo_mas_utilizado_generico(
            &[
                self.usuarios_activos.clone(),
                self.usuarios_cancelados.clone(),
            ]
            .concat(),
        )
    }

    pub fn subscripcion_mas_contratada_generico(usuarios: &[Usuario]) -> Option<TipoSubscripcion> {
        let mut cantidades: HashMap<TipoSubscripcion, usize> = HashMap::new();
        let mut max_cant: usize = 0;
        let mut subscripcion_max: Option<TipoSubscripcion> = None;
        for user in usuarios.iter() {
            for sub in user.subscripciones.iter() {
                let sub_act = sub.tipo_subscripcion.clone();
                let count = cantidades.entry(sub_act.clone()).or_insert(0);
                *count += 1;
                if *count > max_cant {
                    subscripcion_max = Some(sub_act);
                    max_cant = *count;
                }
            }
        }
        subscripcion_max
    }

    pub fn subscripcion_mas_contratada_activos(&self) -> Option<TipoSubscripcion> {
        Self::subscripcion_mas_contratada_generico(&self.usuarios_activos)
    }

    pub fn subscripcion_mas_contratada(&self) -> Option<TipoSubscripcion> {
        Self::subscripcion_mas_contratada_generico(
            &[
                self.usuarios_activos.clone(),
                self.usuarios_cancelados.clone(),
            ]
            .concat(),
        )
    }

    fn persistir(&self) {
        let mut f = File::create("src/tp05/registros/ej05/suscripciones.json").unwrap();
        let registros_serializado = serde_json::to_string_pretty(&self).unwrap();
        f.write_all(registros_serializado.as_bytes()).unwrap();
    }

    fn cargar() -> Self {
        let mut f = match File::open("src/tp05/registros/ej05/suscripciones.json") {
            Ok(file) => file,
            Err(_) => return StreamingRust::new(),
        };
        let mut contenido = String::new();
        f.read_to_string(&mut contenido).unwrap();
        serde_json::from_str(&contenido).unwrap_or_else(|_| StreamingRust::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestData {
        sistema: StreamingRust,
        usuarios: Vec<Usuario>,
    }

    fn setup() -> TestData {
        let mut sr = StreamingRust::new();

        let mut u1 = Usuario::new("tao".to_string(), "tao@example.com".to_string());
        let mut u2 = Usuario::new("clasico".to_string(), "clasico@example.com".to_string());
        let mut u3 = Usuario::new(
            "super cred".to_string(),
            "supercred@example.com".to_string(),
        );
        let mut u4 = Usuario::new(
            "classic cripto".to_string(),
            "classicripto@example.com".to_string(),
        );

        let sub_basic_efectivo = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "tao@example.com".to_string(),
        );
        let sub_classic_efectivo = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::Efectivo,
            TipoSubscripcion::Classic,
            "clasico@example.com".to_string(),
        );
        let sub_super_credito = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::TarjetaCredito {
                num_tarjeta: "123".to_string(),
            },
            TipoSubscripcion::Super,
            "supercred@example.com".to_string(),
        );
        let sub_classic_cripto = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::Cripto {
                direccion: "0x1829821".to_string(),
                moneda: "ETH".to_string(),
            },
            TipoSubscripcion::Classic,
            "classicripto@example.com".to_string(),
        );

        u1.add_subscripcion(sub_basic_efectivo);
        u2.add_subscripcion(sub_classic_efectivo);
        u3.add_subscripcion(sub_super_credito);
        u4.add_subscripcion(sub_classic_cripto);

        sr.crear_usr(&u1);
        sr.crear_usr(&u2);
        sr.crear_usr(&u3);
        sr.crear_usr(&u4);

        TestData {
            sistema: sr,
            usuarios: vec![u1, u2, u3, u4],
        }
    }

    #[test]
    fn test_ej03_upgrade_sub() {
        let mut data = setup();
        data.sistema.upgrade_subscripcion("tao@example.com", 0); // basic -> classic
        data.sistema.upgrade_subscripcion("clasico@example.com", 0); // classic -> super
        let usuarios_post = data.sistema.usuarios_activos.clone();
        assert_eq!(
            usuarios_post[0].subscripciones[0].tipo_subscripcion,
            TipoSubscripcion::Classic
        );
        assert_eq!(
            usuarios_post[1].subscripciones[0].tipo_subscripcion,
            TipoSubscripcion::Super
        );
    }

    #[test]
    fn test_ej03_downgrade_sub() {
        let mut data = setup();
        let usuarios_cancelados_pre = data.sistema.usuarios_cancelados.clone();
        data.sistema.downgrade_subscripcion("tao@example.com", 0); // basic -> cancelado
        data.sistema
            .downgrade_subscripcion("clasico@example.com", 0); // classic -> basic
        let usuarios_post = data.sistema.usuarios_activos.clone();
        let usuarios_cancelados_post = data.sistema.usuarios_cancelados.clone();

        assert!(usuarios_cancelados_pre.is_empty());
        assert!(!usuarios_cancelados_post.is_empty());

        let clasico_user = usuarios_post
            .iter()
            .find(|u| u.email == "clasico@example.com")
            .expect("Usuario 'clasico' deberia existir");
        assert_eq!(clasico_user.nombre, "clasico".to_string());
        assert_eq!(
            clasico_user.subscripciones[0].tipo_subscripcion,
            TipoSubscripcion::Basic
        );
    }

    #[test]
    fn test_ej03_cancelar_sub() {
        let mut data = setup();
        let usuarios_cancelados_pre = data.sistema.usuarios_cancelados.clone();
        data.sistema.cancelar_subscripcion("tao@example.com", 0);
        let usuarios_post = data.sistema.usuarios_activos.clone();
        let usuarios_cancelados_post = data.sistema.usuarios_cancelados.clone();

        assert!(usuarios_cancelados_pre.is_empty());
        assert!(!usuarios_cancelados_post.is_empty());
        assert_eq!(usuarios_cancelados_post[0].nombre, "tao".to_string());
    }

    #[test]
    fn test_ej03_medio_mas_utilizado_activos() {
        let data = setup();
        let medio_max = data.sistema.metodo_mas_utilizado_activos();
        assert_eq!(medio_max.unwrap(), MetodoPago::Efectivo);
    }

    #[test]
    fn test_ej03_medio_mas_utilizado() {
        let mut data = setup();
        data.sistema.cancelar_subscripcion("tao@example.com", 0);
        let medio_max = data.sistema.metodo_mas_utilizado();
        assert_eq!(medio_max.unwrap(), MetodoPago::Efectivo);
    }

    #[test]
    fn test_ej03_sub_mas_contratada_activos() {
        let data = setup();
        let max_sub = data.sistema.subscripcion_mas_contratada_activos();
        assert_eq!(max_sub.unwrap(), TipoSubscripcion::Classic);
    }

    #[test]
    fn test_ej03_sub_mas_contratada() {
        let mut data = setup();
        let max_sub = data.sistema.subscripcion_mas_contratada();
        assert_eq!(max_sub.unwrap(), TipoSubscripcion::Classic);

        let mut u5 = Usuario::new(
            "basic inventado".to_string(),
            "basic1@example.com".to_string(),
        );
        let mut u6 = Usuario::new(
            "basic inventado 2".to_string(),
            "basic2@example.com".to_string(),
        );
        let sub_basic_efectivo = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "basic1@example.com".to_string(),
        );
        let sub_basic_efectivo2 = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "basic2@example.com".to_string(),
        );
        u5.add_subscripcion(sub_basic_efectivo);
        u6.add_subscripcion(sub_basic_efectivo2);
        data.sistema.crear_usr(&u5);
        data.sistema.crear_usr(&u6);

        let max_sub = data.sistema.subscripcion_mas_contratada();
        assert_eq!(max_sub.unwrap(), TipoSubscripcion::Basic);
    }

    #[test]
    fn test_buscar_usuario_existente() {
        let mut data = setup();
        let usuario = &data.usuarios[0];
        let encontrado = data.sistema.buscar_usuario(&usuario.email);
        assert!(encontrado.is_some());
        assert_eq!(encontrado.unwrap().nombre, usuario.nombre);
    }

    #[test]
    fn test_buscar_usuario_inexistente() {
        let mut data = setup();
        let encontrado = data.sistema.buscar_usuario("noexiste@example.com");
        assert!(encontrado.is_none());
    }

    #[test]
    fn test_buscar_usuario_i_existente() {
        let mut data = setup();
        let usuario = &data.usuarios[1];
        let indice = data.sistema.buscar_usuario_i(&usuario.email);
        assert_eq!(indice, Some(1));
    }

    #[test]
    fn test_buscar_usuario_i_inexistente() {
        let mut data = setup();
        let indice = data.sistema.buscar_usuario_i("noexiste@example.com");
        assert!(indice.is_none());
    }

    #[test]
    fn test_upgrade_sub_super_no_cambia() {
        let mut data = setup();
        let usuario = &data.usuarios[2]; // Super
        let tipo_pre = usuario.subscripciones[0].tipo_subscripcion.clone();
        let resultado = data.sistema.upgrade_subscripcion(&usuario.email, 0);
        let usuario_post = data.sistema.buscar_usuario(&usuario.email).unwrap().clone();
        assert!(resultado);
        assert_eq!(usuario_post.subscripciones[0].tipo_subscripcion, tipo_pre);
        assert_eq!(
            usuario_post.subscripciones[0].tipo_subscripcion,
            TipoSubscripcion::Super
        );
    }

    #[test]
    fn test_upgrade_sub_inexistente() {
        let mut data = setup();
        let resultado = data.sistema.upgrade_subscripcion("noexiste@example.com", 0);
        assert!(!resultado);
    }

    #[test]
    fn test_downgrade_sub_super_a_classic() {
        let mut data = setup();
        let usuario = &data.usuarios[2]; // Super
        let resultado = data.sistema.downgrade_subscripcion(&usuario.email, 0);
        let usuario_post = data.sistema.buscar_usuario(&usuario.email).unwrap().clone();
        assert!(resultado);
        assert_eq!(
            usuario_post.subscripciones[0].tipo_subscripcion,
            TipoSubscripcion::Classic
        );
    }

    #[test]
    fn test_downgrade_sub_inexistente() {
        let mut data = setup();
        let resultado = data
            .sistema
            .downgrade_subscripcion("noexiste@example.com", 0);
        assert!(!resultado);
    }

    #[test]
    fn test_cancelar_sub_inexistente() {
        let mut data = setup();
        let resultado = data
            .sistema
            .cancelar_subscripcion("noexiste@example.com", 0);
        assert!(!resultado);
        assert!(data.sistema.usuarios_cancelados.is_empty());
    }

    #[test]
    fn test_metodo_mas_utilizado_activos_vacio() {
        let data = StreamingRust::new();
        let resultado = data.metodo_mas_utilizado_activos();
        assert!(resultado.is_none());
    }

    #[test]
    fn test_metodo_mas_utilizado_vacio() {
        let data = StreamingRust::new();
        let resultado = data.metodo_mas_utilizado();
        assert!(resultado.is_none());
    }

    #[test]
    fn test_sub_mas_contratada_activos_vacio() {
        let data = StreamingRust::new();
        let resultado = data.subscripcion_mas_contratada_activos();
        assert!(resultado.is_none());
    }

    #[test]
    fn test_sub_mas_contratada_vacio() {
        let data = StreamingRust::new();
        let resultado = data.subscripcion_mas_contratada();
        assert!(resultado.is_none());
    }

    #[test]
    fn test_metodo_mas_utilizado_empate() {
        let mut sr = StreamingRust::new();
        let mut u1 = Usuario::new("u1".to_string(), "u1@example.com".to_string());
        let mut u2 = Usuario::new("u2".to_string(), "u2@example.com".to_string());
        let sub1 = Subscripcion::new(
            1000.0,
            6,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "u1@example.com".to_string(),
        );
        let sub2 = Subscripcion::new(
            1000.0,
            6,
            MetodoPago::MercadoPago {
                alias_mp: "alias1".to_string(),
            },
            TipoSubscripcion::Basic,
            "u2@example.com".to_string(),
        );
        u1.add_subscripcion(sub1);
        u2.add_subscripcion(sub2);
        sr.crear_usr(&u1);
        sr.crear_usr(&u2);
        let resultado = sr.metodo_mas_utilizado_activos();
        assert!(resultado.is_some());
    }

    #[test]
    fn test_sub_mas_contratada_empate() {
        let mut sr = StreamingRust::new();
        let mut u1 = Usuario::new("u1".to_string(), "u1@example.com".to_string());
        let mut u2 = Usuario::new("u2".to_string(), "u2@example.com".to_string());
        let sub1 = Subscripcion::new(
            1000.0,
            6,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "u1@example.com".to_string(),
        );
        let sub2 = Subscripcion::new(
            1000.0,
            6,
            MetodoPago::Efectivo,
            TipoSubscripcion::Classic,
            "u2@example.com".to_string(),
        );
        u1.add_subscripcion(sub1);
        u2.add_subscripcion(sub2);
        sr.crear_usr(&u1);
        sr.crear_usr(&u2);
        let resultado = sr.subscripcion_mas_contratada_activos();
        assert!(resultado.is_some());
    }

    #[test]
    fn test_todos_metodos_pago() {
        let mut sr = StreamingRust::new();
        let metodos = vec![
            MetodoPago::Efectivo,
            MetodoPago::MercadoPago {
                alias_mp: "alias1".to_string(),
            },
            MetodoPago::TarjetaCredito {
                num_tarjeta: "123".to_string(),
            },
            MetodoPago::Transferencia {
                cbu: "456".to_string(),
            },
            MetodoPago::Cripto {
                direccion: "0x789".to_string(),
                moneda: "BTC".to_string(),
            },
        ];
        for (i, metodo) in metodos.into_iter().enumerate() {
            let mut usuario =
                Usuario::new(format!("u{}", i), format!("u{}@example.com", i).to_string());
            let sub = Subscripcion::new(
                1000.0,
                6,
                metodo,
                TipoSubscripcion::Basic,
                format!("u{}@example.com", i).to_string(),
            );
            usuario.add_subscripcion(sub);
            sr.crear_usr(&usuario);
        }
        let resultado = sr.metodo_mas_utilizado_activos();
        assert!(resultado.is_some());
        assert_eq!(sr.usuarios_activos.len(), 5);
    }

    #[test]
    fn test_subscripcion_new_fecha_inicio() {
        let sub = Subscripcion::new(
            1000.0,
            6,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "test@example.com".to_string(),
        );
        assert_eq!(sub.fecha_inicio, Fecha::fecha_actual());
    }
}

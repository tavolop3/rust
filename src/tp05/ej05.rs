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
    subscripcion: Subscripcion,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Subscripcion {
    costo_mensual: f32,
    duracion: u8,
    fecha_inicio: Fecha,
    metodo_pago: MetodoPago,
    tipo_subscripcion: TipoSubscripcion,
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
        }
    }
}

impl Usuario {
    pub fn new(nombre: String, subscripcion: Subscripcion) -> Self {
        Usuario {
            nombre,
            subscripcion,
        }
    }
}

impl Subscripcion {
    pub fn new(
        costo_mensual: f32,
        duracion: u8,
        metodo_pago: MetodoPago,
        tipo_subscripcion: TipoSubscripcion,
    ) -> Self {
        Subscripcion {
            costo_mensual,
            duracion,
            fecha_inicio: Fecha::fecha_actual(),
            metodo_pago,
            tipo_subscripcion,
        }
    }
}

impl StreamingRust {
    pub fn new() -> Self {
        StreamingRust {
            usuarios_activos: vec![],
            usuarios_cancelados: vec![],
        }
    }

    pub fn crear_usr(&mut self, usuario: &Usuario) {
        let mut sistema = Self::cargar();
        sistema.usuarios_activos.push(usuario.clone());
        sistema.persistir();
        *self = sistema;
    }

    fn buscar_usuario(&mut self, usuario: &Usuario) -> Option<&mut Usuario> {
        self.usuarios_activos.iter_mut().find(|u| *u == usuario)
    }

    fn buscar_usuario_i(&mut self, usuario: &Usuario) -> Option<usize> {
        self.usuarios_activos.iter_mut().position(|u| u == usuario)
    }

    pub fn upgrade_subscripcion(&mut self, usuario: &Usuario) -> bool {
        let mut sistema = Self::cargar();
        if let Some(us) = sistema.buscar_usuario(usuario) {
            us.subscripcion.tipo_subscripcion.upgrade();
            sistema.persistir();
            *self = sistema;
            return true;
        }
        false
    }

    pub fn cancelar_subscripcion(&mut self, usuario: &Usuario) -> bool {
        let mut sistema = Self::cargar();
        if let Some(i) = self.buscar_usuario_i(usuario) {
            let usuario_cancelado = sistema.usuarios_activos.swap_remove(i);
            sistema.usuarios_cancelados.push(usuario_cancelado);
            sistema.persistir();
            *self = sistema;
            return true;
        }
        false
    }

    pub fn downgrade_subscripcion(&mut self, usuario: &Usuario) -> bool {
        let mut sistema = Self::cargar();
        if let Some(u) = sistema.buscar_usuario(usuario) {
            if u.subscripcion.tipo_subscripcion == TipoSubscripcion::Basic {
                sistema.cancelar_subscripcion(usuario);
            } else {
                u.subscripcion.tipo_subscripcion.downgrade();
                sistema.persistir();
            }
            *self = sistema;
            return true;
        }
        false
    }

    fn metodo_mas_utilizado_generico(coleccion: &[Usuario]) -> Option<MetodoPago> {
        let mut cantidades: HashMap<MetodoPago, usize> = HashMap::new();
        let mut max_cant: usize = 0;
        let mut metodo_max = None;
        for u in coleccion.iter() {
            let metodo_act = u.subscripcion.metodo_pago.clone();
            let count = cantidades.entry(metodo_act.clone()).or_insert(0);
            *count += 1;
            if *count >= max_cant {
                metodo_max = Some(metodo_act);
                max_cant = *count;
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
        for u in usuarios.iter() {
            let sub_act = u.subscripcion.tipo_subscripcion.clone();
            let count = cantidades.entry(sub_act.clone()).or_insert(0);
            *count += 1;
            if *count >= max_cant {
                max_cant = *count;
                subscripcion_max = Some(sub_act.clone());
            }

            // cantidades
            //     .entry(sub_act.clone())
            //     .and_modify(|c| {
            //         *c += 1;
            //         if *c > max_cant {
            //             subscripcion_max = Some(sub_act);
            //             max_cant = *c;
            //         };
            //     })
            //     .or_insert(1);
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
        let sub_basic_efectivo =
            Subscripcion::new(5000.0, 12, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let sub_classic_efectivo =
            Subscripcion::new(5000.0, 12, MetodoPago::Efectivo, TipoSubscripcion::Classic);
        let sub_super_credito = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::TarjetaCredito {
                num_tarjeta: 123.to_string(),
            },
            TipoSubscripcion::Super,
        );
        let sub_classic_cripto = Subscripcion::new(
            5000.0,
            12,
            MetodoPago::Cripto {
                direccion: "0x1829821".to_string(),
                moneda: "ETH".to_string(),
            },
            TipoSubscripcion::Classic,
        );

        let u1 = Usuario::new("tao".to_string(), sub_basic_efectivo);
        let u2 = Usuario::new("clasico".to_string(), sub_classic_efectivo);
        let u3 = Usuario::new("super cred".to_string(), sub_super_credito);
        let u4 = Usuario::new("classic cripto".to_string(), sub_classic_cripto);

        let mut sr = StreamingRust::new();
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
        let usuarios_pre = data.sistema.usuarios_activos.clone();
        data.sistema.upgrade_subscripcion(&usuarios_pre[0]); // basic -> classic
        data.sistema.upgrade_subscripcion(&usuarios_pre[1]); // classic -> super
        let usuarios_post = data.sistema.usuarios_activos.clone();
        assert_eq!(
            usuarios_post[0].subscripcion.tipo_subscripcion,
            TipoSubscripcion::Classic
        );
        assert!(usuarios_post[1].subscripcion.tipo_subscripcion == TipoSubscripcion::Super);
    }

    #[test]
    fn test_ej03_downgrade_sub() {
        let mut data = setup();
        let usuarios_pre = data.usuarios.clone();
        let usuarios_cancelados_pre = data.sistema.usuarios_cancelados.clone();
        data.sistema.downgrade_subscripcion(&usuarios_pre[0]); // basic -> cancelado
        data.sistema.downgrade_subscripcion(&usuarios_pre[1]); // classic -> basic
        let usuarios_post = data.sistema.usuarios_activos.clone();
        let usuarios_cancelados_post = data.sistema.usuarios_cancelados.clone();

        assert!(usuarios_cancelados_pre.is_empty());
        assert!(!usuarios_cancelados_post.is_empty());
        assert_eq!(usuarios_post[1].nombre, "clasico".to_string());
        assert_eq!(
            usuarios_post[1].subscripcion.tipo_subscripcion,
            TipoSubscripcion::Basic
        );
    }

    #[test]
    fn test_ej03_cancelar_sub() {
        let mut data = setup();
        let usuarios_pre = data.usuarios.clone();
        let usuarios_cancelados_pre = data.sistema.usuarios_cancelados.clone();
        data.sistema.cancelar_subscripcion(&usuarios_pre[0]);
        let usuarios_post = data.sistema.usuarios_activos.clone();
        let usuarios_cancelados_post = data.sistema.usuarios_cancelados.clone();

        assert!(usuarios_cancelados_pre.is_empty());
        assert_eq!(usuarios_pre[0].nombre, "tao".to_string());
        assert!(!usuarios_cancelados_post.is_empty());
        assert_ne!(usuarios_post[0].nombre, "tao".to_string());
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
        data.sistema.cancelar_subscripcion(&data.usuarios[0]);
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

        //creo 2 con basic asi es la mas contratada
        let sub_basic_efectivo =
            Subscripcion::new(5000.0, 12, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let u5 = Usuario::new("basic inventado".to_string(), sub_basic_efectivo.clone());
        let u6 = Usuario::new("basic inventado 2".to_string(), sub_basic_efectivo);
        data.sistema.crear_usr(&u5);
        data.sistema.crear_usr(&u6);

        let max_sub = data.sistema.subscripcion_mas_contratada();
        assert_eq!(max_sub.unwrap(), TipoSubscripcion::Basic);
    }

    #[test]
    fn test_buscar_usuario_existente() {
        let mut data = setup();
        let usuario = &data.usuarios[0];
        let encontrado = data.sistema.buscar_usuario(usuario);
        assert!(encontrado.is_some());
        assert_eq!(encontrado.unwrap().nombre, usuario.nombre);
    }

    #[test]
    fn test_buscar_usuario_inexistente() {
        let mut data = setup();
        let sub = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let usuario_inexistente = Usuario::new("no existe".to_string(), sub);
        let encontrado = data.sistema.buscar_usuario(&usuario_inexistente);
        assert!(encontrado.is_none());
    }

    #[test]
    fn test_buscar_usuario_i_existente() {
        let mut data = setup();
        let usuario = &data.usuarios[1];
        let indice = data.sistema.buscar_usuario_i(usuario);
        assert_eq!(indice, Some(1));
    }

    #[test]
    fn test_buscar_usuario_i_inexistente() {
        let mut data = setup();
        let sub = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let usuario_inexistente = Usuario::new("no existe".to_string(), sub);
        let indice = data.sistema.buscar_usuario_i(&usuario_inexistente);
        assert!(indice.is_none());
    }

    #[test]
    fn test_upgrade_sub_super_no_cambia() {
        let mut data = setup();
        let usuario = &data.usuarios[2]; // Super
        let tipo_pre = usuario.subscripcion.tipo_subscripcion.clone();
        let resultado = data.sistema.upgrade_subscripcion(usuario);
        let usuario_post = data.sistema.usuarios_activos[2].clone();
        assert!(resultado);
        assert_eq!(usuario_post.subscripcion.tipo_subscripcion, tipo_pre);
        assert_eq!(
            usuario_post.subscripcion.tipo_subscripcion,
            TipoSubscripcion::Super
        );
    }

    #[test]
    fn test_upgrade_sub_inexistente() {
        let mut data = setup();
        let sub = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let usuario_inexistente = Usuario::new("no existe".to_string(), sub);
        let resultado = data.sistema.upgrade_subscripcion(&usuario_inexistente);
        assert!(!resultado);
    }

    #[test]
    fn test_downgrade_sub_super_a_classic() {
        let mut data = setup();
        let usuario = &data.usuarios[2]; // Super
        let resultado = data.sistema.downgrade_subscripcion(usuario);
        let usuario_post = data.sistema.usuarios_activos[2].clone();
        assert!(resultado);
        assert_eq!(
            usuario_post.subscripcion.tipo_subscripcion,
            TipoSubscripcion::Classic
        );
    }

    #[test]
    fn test_downgrade_sub_inexistente() {
        let mut data = setup();
        let sub = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let usuario_inexistente = Usuario::new("no existe".to_string(), sub);
        let resultado = data.sistema.downgrade_subscripcion(&usuario_inexistente);
        assert!(!resultado);
    }

    #[test]
    fn test_cancelar_sub_inexistente() {
        let mut data = setup();
        let sub = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let usuario_inexistente = Usuario::new("no existe".to_string(), sub);
        let resultado = data.sistema.cancelar_subscripcion(&usuario_inexistente);
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
        let sub1 = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let sub2 = Subscripcion::new(
            1000.0,
            6,
            MetodoPago::MercadoPago {
                alias_mp: "alias1".to_string(),
            },
            TipoSubscripcion::Basic,
        );
        let u1 = Usuario::new("u1".to_string(), sub1);
        let u2 = Usuario::new("u2".to_string(), sub2);
        sr.crear_usr(&u1);
        sr.crear_usr(&u2);
        let resultado = sr.metodo_mas_utilizado_activos();
        assert!(resultado.is_some()); // Comportamiento indefinido, pero no debe fallar
    }

    #[test]
    fn test_sub_mas_contratada_empate() {
        let mut sr = StreamingRust::new();
        let sub1 = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        let sub2 = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Classic);
        let u1 = Usuario::new("u1".to_string(), sub1);
        let u2 = Usuario::new("u2".to_string(), sub2);
        sr.crear_usr(&u1);
        sr.crear_usr(&u2);
        let resultado = sr.subscripcion_mas_contratada_activos();
        assert!(resultado.is_some()); // Comportamiento indefinido, pero no debe fallar
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
            let sub = Subscripcion::new(1000.0, 6, metodo, TipoSubscripcion::Basic);
            let usuario = Usuario::new(format!("u{}", i), sub);
            sr.crear_usr(&usuario);
        }
        let resultado = sr.metodo_mas_utilizado_activos();
        assert!(resultado.is_some()); // Hay empate, pero no debe fallar
        assert_eq!(sr.usuarios_activos.len(), 5);
    }

    #[test]
    fn test_subscripcion_new_fecha_inicio() {
        let sub = Subscripcion::new(1000.0, 6, MetodoPago::Efectivo, TipoSubscripcion::Basic);
        assert_eq!(sub.fecha_inicio, Fecha::fecha_actual());
    }
}

/* ENUNCIADO TP4
La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.

Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.
*/

/*
 * ENUNCIADO TP5
5- En base al ejercicio 3 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todas las suscripciones deben almacenarse en un archivo en formato JSON,
implemente lo necesario para que toda la funcionalidad de las suscripciones se realice
guardando, leyendo o modificando archivos.No debe modificar los tests hechos en el punto
a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto. Recuerde también que se debe seguir manteniendo un coverage de al menos 90%.

un usuario puede tener una o mas subscripciones activas
ccon respecto a esto te hago 2 observaciones:
1- Te conviene por cuestion de modelado que la suscripcion tenga al usuario o una referencia a el, como un id o email por ej
2- en todos los ejercicios del tp 5 vas a persistir siempre todo el sistema, esto lo estuvimos hablando en el canal, y en el new haces la creacion del archivo con la estructura del sistema inicializada
*/

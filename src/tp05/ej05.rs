#![allow(dead_code, unused_variables)]
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::tp03::ej03::Fecha;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct StreamingRust {
    usuarios_activos: Vec<Usuario>,
    usuarios_cancelados: Vec<Usuario>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Usuario {
    nombre: String,
    subscripciones: Vec<Subscripcion>,
    email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Subscripcion {
    costo_mensual: f32,
    duracion: u8,
    fecha_inicio: Fecha,
    metodo_pago: MetodoPago,
    tipo_subscripcion: TipoSubscripcion,
    usr_email: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
pub enum MetodoPago {
    Efectivo,
    MercadoPago { alias_mp: String },
    TarjetaCredito { num_tarjeta: String },
    Transferencia { cbu: String },
    Cripto { direccion: String, moneda: String },
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
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
        StreamingRust {
            usuarios_activos: vec![],
            usuarios_cancelados: vec![],
        }
    }

    pub fn persistir(&self) {
        let mut f = File::create("src/tp05/registros/ej05/streaming_rust.json").unwrap();
        let registros_serializado = serde_json::to_string_pretty(&self).unwrap();
        f.write_all(registros_serializado.as_bytes()).unwrap();
    }

    pub fn crear_usr(&mut self, usuario: &Usuario) {
        self.usuarios_activos.push(usuario.clone());
        self.persistir();
    }

    fn buscar_usuario(&mut self, email: &str) -> Option<&mut Usuario> {
        self.usuarios_activos.iter_mut().find(|u| u.email == email)
    }

    fn buscar_usuario_i(&mut self, email: &str) -> Option<usize> {
        self.usuarios_activos.iter().position(|u| u.email == email)
    }

    pub fn upgrade_subscripcion(&mut self, email: &str, sub_index: usize) -> bool {
        if let Some(user) = self.buscar_usuario(email) {
            if sub_index < user.subscripciones.len() {
                user.subscripciones[sub_index].tipo_subscripcion.upgrade();
                self.persistir();
                return true;
            }
        }
        false
    }

    pub fn cancelar_subscripcion(&mut self, email: &str, sub_index: usize) -> bool {
        if let Some(user) = self.buscar_usuario(email) {
            if sub_index < user.subscripciones.len() {
                if user.subscripciones.len() == 1 {
                    if let Some(i) = self.buscar_usuario_i(email) {
                        let usuario_cancelado = self.usuarios_activos.swap_remove(i);
                        self.usuarios_cancelados.push(usuario_cancelado);
                        self.persistir();
                        return true;
                    }
                } else {
                    user.subscripciones.remove(sub_index);
                    self.persistir();
                    return true;
                }
            }
        }
        false
    }

    pub fn downgrade_subscripcion(&mut self, email: &str, sub_index: usize) -> bool {
        if let Some(user) = self.buscar_usuario(email) {
            if sub_index < user.subscripciones.len() {
                if user.subscripciones[sub_index].tipo_subscripcion == TipoSubscripcion::Basic {
                    let result = self.cancelar_subscripcion(email, sub_index);
                    self.persistir();
                    return result;
                } else {
                    user.subscripciones[sub_index].tipo_subscripcion.downgrade();
                    self.persistir();
                    return true;
                }
            }
        }
        false
    }

    fn metodo_mas_utilizado_generico(coleccion: &[Usuario]) -> Option<MetodoPago> {
        let mut cantidades: HashMap<MetodoPago, usize> = HashMap::new();

        // contar
        for user in coleccion.iter() {
            for sub in user.subscripciones.iter() {
                *cantidades.entry(sub.metodo_pago.clone()).or_insert(0) += 1;
            }
        }

        // encontrar el max
        cantidades
            .into_iter() // Convertir el HM en un iterador de pares (key, value)
            .max_by_key(|&(_, count)| count) // Encuentra el par con el max valor
            .map(|(metodo, _)| metodo) // Tomar solo el MetodoPago (key), descarta el conteo
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

        // contar
        for user in usuarios.iter() {
            for sub in user.subscripciones.iter() {
                *cantidades.entry(sub.tipo_subscripcion.clone()).or_insert(0) += 1;
            }
        }

        // buscar el max
        cantidades
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(subscripcion_type, _)| subscripcion_type)
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
    fn test_ej05_upgrade_sub() {
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
    fn test_ej05_upgrade_sub_super_stays_super() {
        let mut data = setup();
        data.sistema
            .upgrade_subscripcion("supercred@example.com", 0); // super -> super
        let usuarios_post = data.sistema.usuarios_activos.clone();
        let super_user = usuarios_post
            .iter()
            .find(|u| u.email == "supercred@example.com")
            .expect("Usuario 'supercred' debería existir");
        assert_eq!(
            super_user.subscripciones[0].tipo_subscripcion,
            TipoSubscripcion::Super
        );
    }

    #[test]
    fn test_ej05_upgrade_sub_invalid_index() {
        let mut data = setup();
        let result = data.sistema.upgrade_subscripcion("tao@example.com", 99);
        assert!(!result);
    }
    #[test]
    fn test_ej05_upgrade_sub_non_existent_user() {
        let mut data = setup();
        let result = data
            .sistema
            .upgrade_subscripcion("nonexistent@example.com", 0);
        assert!(!result);
    }

    #[test]
    fn test_ej05_downgrade_sub() {
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
    fn test_ej05_downgrade_sub_invalid_index() {
        let mut data = setup();
        let result = data.sistema.downgrade_subscripcion("tao@example.com", 99);
        assert!(!result);
    }
    #[test]
    fn test_ej05_downgrade_sub_non_existent_user() {
        let mut data = setup();
        let result = data
            .sistema
            .downgrade_subscripcion("nonexistent@example.com", 0);
        assert!(!result);
    }

    #[test]
    fn test_ej05_cancelar_sub() {
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
    fn test_ej05_cancelar_sub_multiple_subscriptions() {
        let mut data = setup();
        let mut u_multi = Usuario::new("multi".to_string(), "multi@example.com".to_string());
        u_multi.add_subscripcion(Subscripcion::new(
            100.0,
            1,
            MetodoPago::Efectivo,
            TipoSubscripcion::Basic,
            "multi@example.com".to_string(),
        ));
        u_multi.add_subscripcion(Subscripcion::new(
            200.0,
            1,
            MetodoPago::Efectivo,
            TipoSubscripcion::Classic,
            "multi@example.com".to_string(),
        ));
        data.sistema.crear_usr(&u_multi);
        let initial_active_users_count = data.sistema.usuarios_activos.len();
        let result = data.sistema.cancelar_subscripcion("multi@example.com", 0);
        assert!(result);
        let multi_user = data
            .sistema
            .buscar_usuario("multi@example.com")
            .expect("Usuario 'multi' debería seguir existiendo");
        assert_eq!(multi_user.subscripciones.len(), 1);
        assert_eq!(
            data.sistema.usuarios_activos.len(),
            initial_active_users_count
        );
    }

    #[test]
    fn test_ej05_cancelar_sub_invalid_index() {
        let mut data = setup();
        let result = data.sistema.cancelar_subscripcion("tao@example.com", 99);
        assert!(!result);
    }

    #[test]
    fn test_ej05_cancelar_sub_non_existent_user() {
        let mut data = setup();
        let result = data
            .sistema
            .cancelar_subscripcion("nonexistent@example.com", 0);
        assert!(!result);
    }

    #[test]
    fn test_ej05_medio_mas_utilizado_activos() {
        let data = setup();
        let medio_max = data.sistema.metodo_mas_utilizado_activos();
        assert_eq!(medio_max.unwrap(), MetodoPago::Efectivo);
    }

    #[test]
    fn test_ej05_medio_mas_utilizado() {
        let mut data = setup();
        data.sistema.cancelar_subscripcion("tao@example.com", 0);
        let medio_max = data.sistema.metodo_mas_utilizado();
        assert_eq!(medio_max.unwrap(), MetodoPago::Efectivo);
    }

    #[test]
    fn test_ej05_sub_mas_contratada_activos() {
        let data = setup();
        let max_sub = data.sistema.subscripcion_mas_contratada_activos();
        assert_eq!(max_sub.unwrap(), TipoSubscripcion::Classic);
    }

    #[test]
    fn test_ej05_sub_mas_contratada() {
        let mut data = setup();
        let max_sub = data.sistema.subscripcion_mas_contratada();
        assert_eq!(max_sub.unwrap(), TipoSubscripcion::Classic);

        // Agregar dos usuarios con subs basicas
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

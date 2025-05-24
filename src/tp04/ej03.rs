#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use crate::tp03::ej03::Fecha;

pub struct StreamingRust {
    usuarios_activos: Vec<Usuario>,
    usuarios_cancelados: Vec<Usuario>,
}

#[derive(PartialEq, Clone)]
pub struct Usuario {
    nombre: String,
    subscripcion: Subscripcion,
}

#[derive(PartialEq, Clone)]
pub struct Subscripcion {
    costo_mensual: f32,
    duracion: u8,
    fecha_inicio: Fecha,
    metodo_pago: MetodoPago,
    tipo_subscripcion: TipoSubscripcion,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum MetodoPago {
    Efectivo,
    MercadoPago { alias_mp: String },
    TarjetaCredito { num_tarjeta: String },
    Transferencia { cbu: String },
    Cripto { direccion: String, moneda: String },
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
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
        self.usuarios_activos.push(usuario.clone());
    }

    fn buscar_usuario(&mut self, usuario: &Usuario) -> Option<&mut Usuario> {
        self.usuarios_activos.iter_mut().find(|u| *u == usuario)
    }

    fn buscar_usuario_i(&mut self, usuario: &Usuario) -> Option<usize> {
        self.usuarios_activos.iter_mut().position(|u| u == usuario)
    }

    pub fn upgrade_subscripcion(&mut self, usuario: &Usuario) -> bool {
        if let Some(us) = self.buscar_usuario(usuario) {
            us.subscripcion.tipo_subscripcion.upgrade();
            return true;
        }
        false
    }

    pub fn cancelar_subscripcion(&mut self, usuario: &Usuario) -> bool {
        if let Some(i) = self.buscar_usuario_i(usuario) {
            let usuario_cancelado = self.usuarios_activos.swap_remove(i);
            self.usuarios_cancelados.push(usuario_cancelado);
            return true;
        }
        false
    }

    pub fn downgrade_subscripcion(&mut self, usuario: &Usuario) -> bool {
        if let Some(u) = self.buscar_usuario(usuario) {
            if u.subscripcion.tipo_subscripcion == TipoSubscripcion::Basic {
                self.cancelar_subscripcion(usuario);
            } else {
                u.subscripcion.tipo_subscripcion.downgrade();
            }
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
            cantidades
                .entry(metodo_act.clone())
                .and_modify(|c| {
                    *c += 1;
                    if *c > max_cant {
                        metodo_max = Some(metodo_act);
                        max_cant = *c;
                    };
                })
                .or_insert(1);
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
            cantidades
                .entry(sub_act.clone())
                .and_modify(|c| {
                    *c += 1;
                    if *c > max_cant {
                        subscripcion_max = Some(sub_act);
                        max_cant = *c;
                    };
                })
                .or_insert(1);
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
}

// La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
// (Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
// duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
// suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
// Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
// correspondientes a excepción de Efectivo.
// Los usuarios solo pueden tener una suscripción activa a la vez.
//
// Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
// siguientes acciones:
// ➢ Crear un usuario con una determinada suscripción y medio de pago.
// ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
// pasa a Clasic y si está en Clasic pasa a Super.
// ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
// suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
// ➢ Dado un usuario cancelar la suscripción.
// ➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
// suscripciones activas
// ➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
// activas.
// ➢ Saber cuál fue el medio de pago más utilizado.
// ➢ Saber cuál fue la suscripción más contratada.

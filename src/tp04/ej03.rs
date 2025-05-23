#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use crate::tp03::ej03::Fecha;

struct StreamingRust {
    usuarios_activos: Vec<Usuario>,
    usuarios_cancelados: Vec<Usuario>,
}

#[derive(PartialEq)]
struct Usuario {
    nombre: String,
    subscripcion: Subscripcion,
}

#[derive(PartialEq)]
struct Subscripcion {
    costo_mensual: f32,
    duarcion: u8,
    fecha_inicio: Fecha,
    metodo_pago: MetodoPago,
    tipo_subscripcion: TipoSubscripcion,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum MetodoPago {
    Efectivo,
    MercadoPago { alias_mp: String },
    TarjetaCredito { num_tarjeta: String },
    Transferencia { cbu: String },
    Cripto { direccion: String, moneda: String },
}

#[derive(PartialEq)]
enum TipoSubscripcion {
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

impl StreamingRust {
    pub fn crear_usr(&mut self, nombre: String, subscripcion: Subscripcion) {
        self.usuarios_activos.push(Usuario {
            nombre,
            subscripcion,
        })
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

    pub fn metodo_mas_utilizado(&self) -> Option<MetodoPago> {
        let mut cantidades: HashMap<MetodoPago, usize> = HashMap::new();
        let mut max_cant: usize = 0;
        let mut metodo_max = None;
        for u in self.usuarios_activos.iter() {
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
}

// cantidades
//     .entry(u.subscripcion.metodo_pago.clone())
//     .and_modify(|c| *c += 1)
//     .or_insert(1);

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

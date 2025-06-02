#![allow(dead_code, unused_variables)]
use crate::tp03::ej03::Fecha;
use std::collections::HashMap;

struct Sistema {
    usuarios: Vec<Usuario>,
    cotizaciones: HashMap<String, f64>,
    criptomonedas: Vec<Criptomoneda>,
    transacciones: Vec<Transaccion>,
}

#[derive(Clone, Debug)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: u32,
    validado: bool,
    monto_fiat: f64,
    balance_criptos: HashMap<String, f64>,
}

#[derive(Clone, Debug)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    blockchains_soportadas: Vec<Blockchain>,
}

#[derive(Clone, PartialEq, Debug)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Debug)]
struct Transaccion {
    fecha: Fecha,
    tipo: TipoTransaccion,
    monto: f64,
    usuario: Usuario,
    criptomoneda: Option<Criptomoneda>,
    cotizacion: Option<f64>,
}

#[derive(Debug)]
enum TipoTransaccion {
    IngresoFiat,
    IngresoCripto,
    RetiroFiat,
    RetiroCripto,
    Compra,
    Venta,
}

impl Transaccion {
    fn new(
        fecha: Fecha,
        tipo: TipoTransaccion,
        monto: f64,
        usuario: Usuario,
        criptomoneda: Option<Criptomoneda>,
        cotizacion: Option<f64>,
    ) -> Self {
        Transaccion {
            fecha,
            tipo,
            monto,
            usuario,
            criptomoneda,
            cotizacion,
        }
    }
}

impl Usuario {
    fn new(
        nombre: String,
        apellido: String,
        email: String,
        dni: u32,
        monto_fiat: f64,
        validado: bool,
    ) -> Self {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            validado,
            monto_fiat,
            balance_criptos: HashMap::new(),
        }
    }
}

impl Sistema {
    fn new(
        usuarios: Vec<Usuario>,
        cotizaciones: HashMap<String, f64>,
        criptomonedas: Vec<Criptomoneda>,
    ) -> Self {
        Sistema {
            usuarios,
            cotizaciones,
            criptomonedas,
            transacciones: vec![],
        }
    }

    fn agregar_criptomoneda(&mut self, criptomoneda: &Criptomoneda) {
        self.criptomonedas.push(criptomoneda.clone());
    }

    fn agregar_usuario(&mut self, usuario: &Usuario) {
        self.usuarios.push(usuario.clone());
    }

    fn buscar_usuario(&mut self, email: String) -> Option<&mut Usuario> {
        self.usuarios.iter_mut().find(|u| u.email == email)
    }

    /*
    ➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
    fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
    que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.
    */
    fn ingresar_dinero(&mut self, monto_fiat: f64, usuario: &Usuario) -> bool {
        if let Some(u) = self.buscar_usuario(usuario.email.clone()) {
            u.monto_fiat += monto_fiat;

            let transaccion = Transaccion::new(
                Fecha::fecha_actual(),
                TipoTransaccion::IngresoFiat,
                monto_fiat,
                usuario.clone(),
                None,
                None,
            );

            self.transacciones.push(transaccion);

            return true;
        }
        false
    }

    /*
    ➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
    de determinada criptomoneda, tenga en cuenta que al momento de realizar la
    operación se obtiene del sistema la cotización actual de la criptomoneda para
    acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
    el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
    fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
    */
    fn comprar_criptomoneda(
        &mut self,
        monto_fiat: f64,
        cantidad: f64,
        criptomoneda: &Criptomoneda,
        usuario: &Usuario,
    ) -> bool {
        if let Some(cotizacion) = self.cotizaciones.get(&criptomoneda.prefijo) {
            let costo = cotizacion * cantidad;
            if costo > monto_fiat {
                return false;
            }

            let transaccion = Transaccion::new(
                Fecha::fecha_actual(),
                TipoTransaccion::Compra,
                monto_fiat,
                usuario.clone(),
                Some(criptomoneda.clone()),
                Some(*cotizacion),
            );

            if let Some(usr) = self.buscar_usuario(usuario.email.clone()) {
                if !usr.validado {
                    return usr.validado;
                }

                let cant_act = usr
                    .balance_criptos
                    .get(&criptomoneda.prefijo)
                    .unwrap_or(&0.0);
                let cant_final: f64 = cant_act + cantidad;
                usr.balance_criptos
                    .insert(criptomoneda.nombre.clone(), cant_final);
                usr.monto_fiat -= costo;
            } else {
                return false;
            };

            self.transacciones.push(transaccion);
            return true;
        };
        false
    }

    // ➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
    // en cuenta que al momento de realizar la operación se obtiene del sistema la
    // cotización actual de la criptomoneda para acreditar la correspondiente proporción en
    // el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
    // registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
    // venta de cripto,monto de cripto y cotización.
    fn vender_criptomoneda(
        &mut self,
        criptomoneda: &Criptomoneda,
        monto_cripto: f64,
        usuario: &Usuario,
    ) -> bool {
        if let Some(cotizacion) = self.cotizaciones.get(&criptomoneda.prefijo) {
            let costo = monto_cripto * cotizacion;
            if costo > usuario.monto_fiat {
                return false;
            }

            let transaccion = Transaccion::new(
                Fecha::fecha_actual(),
                TipoTransaccion::Venta,
                costo,
                usuario.clone(),
                Some(criptomoneda.clone()),
                Some(*cotizacion),
            );

            if let Some(usr) = self.buscar_usuario(usuario.email.clone()) {
                if !usr.validado {
                    return usr.validado;
                }

                let cant_act = usr
                    .balance_criptos
                    .get(&criptomoneda.prefijo)
                    .unwrap_or(&0.0);
                let cant_final: f64 = cant_act - costo;
                if cant_final < 0.0 {
                    return false;
                }

                usr.balance_criptos
                    .insert(criptomoneda.nombre.clone(), cant_final);
                usr.monto_fiat += costo;
            } else {
                return false;
            }

            self.transacciones.push(transaccion);
            true
        } else {
            false
        }
    }

    //➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
    // le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
    // un hash que representa una transacción en ella (esto hágalo retornando el nombre
    // de la blockchain + un número random). Luego se genera una transacción con los
    // siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
    // cotización.
    fn retirar_criptomoneda() {}
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestData {
        sistema: Sistema,
        usuarios: Vec<Usuario>,
        criptomonedas: Vec<Criptomoneda>,
    }

    fn setup() -> TestData {
        let u1 = Usuario::new(
            "tao".to_string(),
            "lop".to_string(),
            "a@a.com".to_string(),
            12345678,
            500.0,
            true,
        );
        let xmr = Criptomoneda {
            prefijo: "Monero".to_string(),
            nombre: "XMR".to_string(),
            blockchains_soportadas: vec![Blockchain {
                nombre: "Monero".to_string(),
                prefijo: "MNR".to_string(),
            }],
        };
        let mut hm: HashMap<String, f64> = HashMap::new();
        hm.insert(xmr.prefijo.clone(), 300.0);

        let s = Sistema::new(vec![u1.clone()], hm, vec![xmr.clone()]);
        TestData {
            sistema: s,
            usuarios: vec![u1],
            criptomonedas: vec![xmr.clone()],
        }
    }

    #[test]
    fn test_ej05_ingresar_dinero() {
        let td = setup();
        let u1 = td.usuarios[0].clone();
        let mut s = td.sistema;
        assert!(s.ingresar_dinero(100.0, &u1));
        assert_eq!(
            s.buscar_usuario("a@a.com".to_string()).unwrap().monto_fiat,
            600.0
        );
        assert!(s.ingresar_dinero(200.0, &u1));
        assert_eq!(
            s.buscar_usuario("a@a.com".to_string()).unwrap().monto_fiat,
            800.0
        );
    }

    #[test]
    fn test_ej05_comprar_criptomoneda() {
        let td = setup();
        let mut s = td.sistema;
        let u = td.usuarios[0].clone();

        assert!(s.comprar_criptomoneda(400.0, 1.0, &td.criptomonedas[0].clone(), &u));
        let u_modificado = s.usuarios[0].clone();
        assert_eq!(u_modificado.monto_fiat, 200.0);
        assert!(!s.transacciones.is_empty());
    }
}

/*
5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de
usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat. De los
usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. De las
criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
enviar o recibir. De cada blockchain se conoce el nombre, prefijo.

Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones relacionadas al usuario:

➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
de determinada criptomoneda, tenga en cuenta que al momento de realizar la
operación se obtiene del sistema la cotización actual de la criptomoneda para
acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.

➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
en cuenta que al momento de realizar la operación se obtiene del sistema la
cotización actual de la criptomoneda para acreditar la correspondiente proporción en
el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
venta de cripto,monto de cripto y cotización.

➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
un hash que representa una transacción en ella (esto hágalo retornando el nombre
de la blockchain + un número random). Luego se genera una transacción con los
siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
cotización.

➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una
transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
blockchain, cripto, monto, cotización.

➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
monto del balance al usuario y se genera una transacción con la siguiente
información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
o Transferencia Bancaria)

Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
Se debe validar siempre que haya balance suficiente para realizar la operación en los casos
de compra, venta, retiro.

Además la empresa desea saber lo siguiente en base a sus operaciones:
➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene
*/

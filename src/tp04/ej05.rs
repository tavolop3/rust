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
    monto_fiat: f64,
    usuario: Usuario,
}

#[derive(Debug)]
enum TipoTransaccion {
    IngresoFiat,
    CompraCripto {
        cripto: String,
        monto_cripto: f64,
        cotizacion: f64,
    },
    VentaCripto {
        cripto: String,
        monto_cripto: f64,
        cotizacion: f64,
    },
    RetiroCripto {
        cripto: String,
        blockchain: String,
        hash: String,
        monto: f64,
        cotizacion: f64,
    },
    RecepcionCripto {
        cripto: String,
        blockchain: String,
        monto: f64,
        cotizacion: f64,
    },
    RetiroFiat {
        medio: MedioRetiroFiat,
    },
}

#[derive(Debug)]
enum MedioRetiroFiat {
    MercadoPago,
    TransferenciaBancaria,
}

impl Blockchain {
    fn generar_hash(&self) -> String {
        format!("{}{}", self.prefijo, rand::random::<u64>())
    }
}

impl Transaccion {
    fn new(fecha: Fecha, tipo: TipoTransaccion, monto_fiat: f64, usuario: Usuario) -> Self {
        Transaccion {
            fecha,
            tipo,
            monto_fiat,
            usuario,
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
    fn ingresar_dinero(&mut self, monto_fiat: f64, usuario: &Usuario) -> Result<(), String> {
        let u = self
            .buscar_usuario(usuario.email.clone())
            .ok_or_else(|| format!("El usuario {} no fue encontrado", usuario.email))?;
        u.monto_fiat += monto_fiat;

        let transaccion = Transaccion::new(
            Fecha::fecha_actual(),
            TipoTransaccion::IngresoFiat,
            monto_fiat,
            usuario.clone(),
        );

        self.transacciones.push(transaccion);

        Ok(())
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
    ) -> Result<(), String> {
        let cotizacion = self
            .cotizaciones
            .get(&criptomoneda.prefijo)
            .ok_or_else(|| format!("No se econtró una cotizacion para {}", criptomoneda.prefijo))?;

        let costo = cotizacion * cantidad;
        if costo > monto_fiat {
            return Err(format!(
                "El costo de la operación supera el monto de fiat {} < {}",
                monto_fiat, costo,
            ));
        }

        let transaccion = Transaccion::new(
            Fecha::fecha_actual(),
            TipoTransaccion::CompraCripto {
                cripto: criptomoneda.prefijo.clone(),
                monto_cripto: cantidad,
                cotizacion: *cotizacion,
            },
            monto_fiat,
            usuario.clone(),
        );

        let usr = self
            .buscar_usuario(usuario.email.clone())
            .ok_or_else(|| format!("El usuario {} no fue encontrado", usuario.email))?;

        if !usr.validado {
            return Err("El usuario no está validado".to_string());
        }

        if usr.monto_fiat < costo {
            return Err(format!(
                "El costo de la operación supera el monto de fiat del usuario {} < {}",
                usr.monto_fiat, costo,
            ));
        }

        let cant_act = usr
            .balance_criptos
            .get(&criptomoneda.prefijo)
            .unwrap_or(&0.0);
        let cant_final: f64 = cant_act + cantidad;
        usr.balance_criptos
            .insert(criptomoneda.prefijo.clone(), cant_final);
        usr.monto_fiat -= costo;

        self.transacciones.push(transaccion);
        Ok(())
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
    ) -> Result<(), String> {
        let cotizacion = self
            .cotizaciones
            .get(&criptomoneda.prefijo)
            .ok_or_else(|| format!("No se econtró una cotizacion para {}", criptomoneda.prefijo))?;

        let costo = monto_cripto * cotizacion;

        let transaccion = Transaccion::new(
            Fecha::fecha_actual(),
            TipoTransaccion::VentaCripto {
                cripto: criptomoneda.prefijo.clone(),
                monto_cripto,
                cotizacion: *cotizacion,
            },
            costo,
            usuario.clone(),
        );

        let usr = self
            .buscar_usuario(usuario.email.clone())
            .ok_or_else(|| format!("El usuario {} no fue encontrado", usuario.email))?;
        if !usr.validado {
            return Err("El usuario no está validado".to_string());
        }

        let cant_act = usr
            .balance_criptos
            .get(&criptomoneda.prefijo)
            .unwrap_or(&0.0);
        if monto_cripto > *cant_act {
            return Err(format!(
                "La cantidad de cripto no puede superar a la del usuario: {} < {}",
                cant_act, monto_cripto
            ));
        }

        let cant_final: f64 = cant_act - monto_cripto;
        usr.balance_criptos
            .insert(criptomoneda.prefijo.clone(), cant_final);
        usr.monto_fiat += costo;

        self.transacciones.push(transaccion);
        Ok(())
    }

    //➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
    // le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
    // un hash que representa una transacción en ella (esto hágalo retornando el nombre
    // de la blockchain + un número random). Luego se genera una transacción con los
    // siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
    // cotización.
    fn retirar_criptomoneda(
        &mut self,
        usuario: &Usuario,
        criptomoneda: &Criptomoneda,
        cantidad: f64,
        blockchain: &Blockchain,
    ) -> Result<(), String> {
        let cotizacion = self
            .cotizaciones
            .get(&criptomoneda.prefijo)
            .ok_or_else(|| format!("No se econtró una cotizacion para {}", criptomoneda.prefijo))?;

        let hash = blockchain.generar_hash();

        let transaccion = Transaccion::new(
            Fecha::fecha_actual(),
            TipoTransaccion::RetiroCripto {
                cripto: criptomoneda.prefijo.clone(),
                blockchain: blockchain.prefijo.clone(),
                hash,
                monto: cantidad,
                cotizacion: *cotizacion,
            },
            cantidad,
            usuario.clone(),
        );

        let usr = self
            .buscar_usuario(usuario.email.clone())
            .ok_or_else(|| format!("El usuario {} no fue encontrado", usuario.email))?;

        if !usr.validado {
            return Err("El usuario no está validado".to_string());
        }

        let cant_act = usr
            .balance_criptos
            .get(&criptomoneda.prefijo)
            .unwrap_or(&0.0);
        if cantidad > *cant_act {
            return Err(format!("Balance insuficiente: {} < {}", cant_act, cantidad));
        }

        let cant_final = cant_act - cantidad;
        usr.balance_criptos
            .insert(criptomoneda.prefijo.clone(), cant_final);

        self.transacciones.push(transaccion);
        Ok(())
    }

    // ➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
    // monto del balance al usuario y se genera una transacción con la siguiente
    // información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
    // o Transferencia Bancaria)
    fn retirar_fiat(
        &mut self,
        usuario: &Usuario,
        monto_fiat: f64,
        medio: MedioRetiroFiat,
    ) -> Result<(), String> {
        let usr = self
            .buscar_usuario(usuario.email.clone())
            .ok_or_else(|| format!("El usuario {} no fue encontrado", usuario.email))?;

        if monto_fiat > usr.monto_fiat {
            return Err(format!(
                "El usuario no tiene el monto requerido: {} < {}",
                usr.monto_fiat, monto_fiat
            ));
        }

        usr.monto_fiat -= monto_fiat;

        let transaccion = Transaccion::new(
            Fecha::fecha_actual(),
            TipoTransaccion::RetiroFiat { medio },
            monto_fiat,
            usuario.clone(),
        );
        self.transacciones.push(transaccion);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestData {
        sistema: Sistema,
        usuarios: Vec<Usuario>,
        criptomonedas: Vec<Criptomoneda>,
        blockchains: Vec<Blockchain>,
    }

    fn setup() -> TestData {
        let u1 = Usuario::new(
            "Tao".to_string(),
            "Lop".to_string(),
            "a@a.com".to_string(),
            12345678,
            500.0,
            true,
        );
        let u2 = Usuario::new(
            "Ana".to_string(),
            "Gomez".to_string(),
            "b@b.com".to_string(),
            87654321,
            1000.0,
            false, // Usuario no validado
        );
        let blockchain = Blockchain {
            nombre: "Monero".to_string(),
            prefijo: "MNR".to_string(),
        };
        let xmr = Criptomoneda {
            prefijo: "XMR".to_string(),
            nombre: "Monero".to_string(),
            blockchains_soportadas: vec![blockchain.clone()],
        };
        let mut hm: HashMap<String, f64> = HashMap::new();
        hm.insert(xmr.prefijo.clone(), 300.0);

        let mut s = Sistema::new(vec![u1.clone(), u2.clone()], hm, vec![xmr.clone()]);
        s.buscar_usuario("a@a.com".to_string())
            .unwrap()
            .balance_criptos
            .insert("XMR".to_string(), 2.0);

        TestData {
            sistema: s,
            usuarios: vec![u1, u2],
            criptomonedas: vec![xmr],
            blockchains: vec![blockchain],
        }
    }

    #[test]
    fn test_ingresar_dinero_exitoso() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();

        assert!(s.ingresar_dinero(100.0, &u1).is_ok());
        assert_eq!(
            s.buscar_usuario("a@a.com".to_string()).unwrap().monto_fiat,
            600.0
        );
        assert_eq!(s.transacciones.len(), 1);
        assert!(matches!(
            s.transacciones[0].tipo,
            TipoTransaccion::IngresoFiat
        ));
        assert_eq!(s.transacciones[0].monto_fiat, 100.0);
    }

    #[test]
    fn test_ingresar_dinero_usuario_no_encontrado() {
        let mut s = setup().sistema;
        let user = Usuario::new(
            "No".to_string(),
            "Exist".to_string(),
            "c@c.com".to_string(),
            99999999,
            0.0,
            true,
        );
        assert_eq!(
            s.ingresar_dinero(100.0, &user).unwrap_err(),
            "El usuario c@c.com no fue encontrado"
        );
    }

    #[test]
    fn test_comprar_criptomoneda_exitoso() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = td.criptomonedas[0].clone();

        assert!(s.comprar_criptomoneda(400.0, 1.0, &xmr, &u1).is_ok());
        let u_mod = s.buscar_usuario("a@a.com".to_string()).unwrap();
        assert_eq!(u_mod.monto_fiat, 200.0); // 500 - (1 * 300)
        assert_eq!(u_mod.balance_criptos.get("XMR").unwrap(), &3.0); // 2 + 1
        assert_eq!(s.transacciones.len(), 1);
        assert!(matches!(
            s.transacciones[0].tipo,
            TipoTransaccion::CompraCripto { monto_cripto, .. } if monto_cripto == 1.0
        ));
    }

    #[test]
    fn test_comprar_criptomoneda_fiat_insuficiente() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = td.criptomonedas[0].clone();

        assert_eq!(
            s.comprar_criptomoneda(200.0, 1.0, &xmr, &u1).unwrap_err(),
            "El costo de la operación supera el monto de fiat 200 < 300"
        );
    }

    #[test]
    fn test_comprar_criptomoneda_no_validado() {
        let td = setup();
        let mut s = td.sistema;
        let u2 = td.usuarios[1].clone(); // Unvalidated user
        let xmr = td.criptomonedas[0].clone();

        assert_eq!(
            s.comprar_criptomoneda(400.0, 1.0, &xmr, &u2).unwrap_err(),
            "El usuario no está validado"
        );
    }

    #[test]
    fn test_comprar_criptomoneda_no_cotizacion() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = Criptomoneda {
            prefijo: "BTC".to_string(),
            nombre: "Bitcoin".to_string(),
            blockchains_soportadas: vec![],
        };

        assert_eq!(
            s.comprar_criptomoneda(400.0, 1.0, &xmr, &u1).unwrap_err(),
            "No se econtró una cotizacion para BTC"
        );
    }

    #[test]
    fn test_vender_criptomoneda_exitoso() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = td.criptomonedas[0].clone();

        assert!(s.vender_criptomoneda(&xmr, 1.0, &u1).is_ok());
        let u_mod = s.buscar_usuario("a@a.com".to_string()).unwrap();
        assert_eq!(u_mod.monto_fiat, 800.0); // 500 + (1 * 300)
        assert_eq!(u_mod.balance_criptos.get("XMR").unwrap(), &1.0); // 2 - 1
        assert_eq!(s.transacciones.len(), 1);
        assert!(matches!(
            s.transacciones[0].tipo,
            TipoTransaccion::VentaCripto { monto_cripto, .. } if monto_cripto == 1.0
        ));
    }

    #[test]
    fn test_vender_criptomoneda_cripto_insuficiente() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = td.criptomonedas[0].clone();

        assert_eq!(
            s.vender_criptomoneda(&xmr, 3.0, &u1).unwrap_err(),
            "La cantidad de cripto no puede superar a la del usuario: 2 < 3",
        );
    }

    #[test]
    fn test_vender_criptomoneda_no_validado() {
        let td = setup();
        let mut s = td.sistema;
        let u2 = td.usuarios[1].clone(); // Usuario no validado
        let xmr = td.criptomonedas[0].clone();

        assert_eq!(
            s.vender_criptomoneda(&xmr, 1.0, &u2).unwrap_err(),
            "El usuario no está validado"
        );
    }

    #[test]
    fn test_vender_criptomoneda_no_cotizacion() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = Criptomoneda {
            prefijo: "BTC".to_string(),
            nombre: "Bitcoin".to_string(),
            blockchains_soportadas: vec![],
        };

        assert_eq!(
            s.vender_criptomoneda(&xmr, 1.0, &u1).unwrap_err(),
            "No se econtró una cotizacion para BTC"
        );
    }

    #[test]
    fn test_retirar_criptomoneda_exitoso() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = td.criptomonedas[0].clone();
        let blockchain = td.blockchains[0].clone();

        assert!(s.retirar_criptomoneda(&u1, &xmr, 1.0, &blockchain).is_ok());
        let u_mod = s.buscar_usuario("a@a.com".to_string()).unwrap();
        assert_eq!(u_mod.balance_criptos.get("XMR").unwrap(), &1.0); // 2 - 1
        assert_eq!(u_mod.monto_fiat, 500.0); // Sin cambios
        assert_eq!(s.transacciones.len(), 1);
        assert!(matches!(
            s.transacciones[0].tipo,
            TipoTransaccion::RetiroCripto { monto, .. } if monto == 1.0
        ));
    }

    #[test]
    fn test_retirar_criptomoneda_cripto_insuficiente() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = td.criptomonedas[0].clone();
        let blockchain = td.blockchains[0].clone();

        assert_eq!(
            s.retirar_criptomoneda(&u1, &xmr, 3.0, &blockchain)
                .unwrap_err(),
            "Balance insuficiente: 2 < 3"
        );
    }

    #[test]
    fn test_retirar_criptomoneda_no_validado() {
        let td = setup();
        let mut s = td.sistema;
        let u2 = td.usuarios[1].clone(); // Usuario no validado
        let xmr = td.criptomonedas[0].clone();
        let blockchain = td.blockchains[0].clone();

        assert_eq!(
            s.retirar_criptomoneda(&u2, &xmr, 1.0, &blockchain)
                .unwrap_err(),
            "El usuario no está validado"
        );
    }

    #[test]
    fn test_retirar_criptomoneda_no_cotizacion() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();
        let xmr = Criptomoneda {
            prefijo: "BTC".to_string(),
            nombre: "Bitcoin".to_string(),
            blockchains_soportadas: vec![td.blockchains[0].clone()],
        };
        let blockchain = td.blockchains[0].clone();

        assert_eq!(
            s.retirar_criptomoneda(&u1, &xmr, 1.0, &blockchain)
                .unwrap_err(),
            "No se econtró una cotizacion para BTC"
        );
    }

    #[test]
    fn test_retirar_fiat_exitoso() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();

        assert!(
            s.retirar_fiat(&u1, 200.0, MedioRetiroFiat::MercadoPago)
                .is_ok()
        );
        let u_mod = s.buscar_usuario("a@a.com".to_string()).unwrap();
        assert_eq!(u_mod.monto_fiat, 300.0); // 500 - 200
        assert_eq!(s.transacciones.len(), 1);
        assert!(matches!(
            s.transacciones[0].tipo,
            TipoTransaccion::RetiroFiat {
                medio: MedioRetiroFiat::MercadoPago
            }
        ));
        assert_eq!(s.transacciones[0].monto_fiat, 200.0);
    }

    #[test]
    fn test_retirar_fiat_fiat_insuficiente() {
        let td = setup();
        let mut s = td.sistema;
        let u1 = td.usuarios[0].clone();

        assert_eq!(
            s.retirar_fiat(&u1, 600.0, MedioRetiroFiat::MercadoPago)
                .unwrap_err(),
            "El usuario no tiene el monto requerido: 500 < 600"
        );
    }

    #[test]
    fn test_retirar_fiat_usuario_inexistente() {
        let mut s = setup().sistema;
        let user = Usuario::new(
            "No".to_string(),
            "Exist".to_string(),
            "c@c.com".to_string(),
            99999999,
            0.0,
            true,
        );

        assert_eq!(
            s.retirar_fiat(&user, 100.0, MedioRetiroFiat::MercadoPago)
                .unwrap_err(),
            "El usuario c@c.com no fue encontrado"
        );
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

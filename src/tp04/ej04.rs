#![allow(dead_code, unused_variables)]

use crate::tp03::ej03::Fecha;
use std::collections::HashMap;
const DESCUENTO_GRAL_NEWSLETTER: u32 = 10;

struct Sistema {
    productos: Vec<Producto>,
    categorias: HashMap<String, u32>,
    ventas: Vec<Venta>,
}

#[derive(Debug, Clone, PartialEq)]
struct Producto {
    nombre: String,
    nombre_categoria: String,
    precio_base: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Venta {
    vendedor: Persona,
    cliente: Persona,
    medio_pago: MedioPago,
    precio_final: Option<f64>, // el precio final puede o no estar computado
    fecha: Fecha,
    registros: Vec<Registro>,
}

#[derive(Debug, Clone, PartialEq)]
struct Registro {
    producto: Producto,
    cantidad: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u64,
    legajo: u64,
    antiguedad: u8,
    salario: f64,
    email_newsletter: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum MedioPago {
    TarjetaCredito,
    TarjetaDebito,
    Transferencia,
    Efectivo,
}

impl Sistema {
    fn new() -> Self {
        Sistema {
            productos: vec![],
            categorias: HashMap::new(),
            ventas: vec![],
        }
    }

    // ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
    // productos con sus cantidades.
    fn crear_venta(
        &mut self,
        fecha: Fecha,
        cliente: Persona,
        vendedor: Persona,
        medio_pago: MedioPago,
        precio_final: Option<f64>,
        registros: Vec<Registro>,
    ) -> Venta {
        let venta = Venta {
            vendedor,
            cliente,
            medio_pago,
            precio_final,
            fecha,
            registros,
        };
        self.ventas.push(venta.clone());
        venta
    }

    //➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
    // calcularlo tenga en cuenta que pueden haber determinados productos de alguna
    // categoría donde debería aplicarse un descuento. Tanto la categoría como el
    // porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
    // sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
    // aplicar un porcentaje de descuento general si el cliente tiene suscripción al
    // newsletter.
    fn calcular_precio_final(&mut self, venta_param: &Venta) -> Option<f64> {
        let venta = self.buscar_venta(venta_param)?; //devuelve none si es none
        let mut subtotal = 0.0;

        for r in &venta.registros {
            let p_descuento = self
                .categorias
                .get(&r.producto.nombre_categoria)
                .cloned()
                .unwrap_or(0);

            let precio_base = r.producto.precio_base;
            let precio_con_descuento = precio_base * (1.0 - p_descuento as f64 / 100.0);
            subtotal += precio_con_descuento * r.cantidad as f64;
        }

        let descuento_newsletter = if venta.cliente.email_newsletter.is_some() {
            DESCUENTO_GRAL_NEWSLETTER as f64
        } else {
            0.0
        };
        let precio_final = subtotal * (1.0 - descuento_newsletter / 100.0);
        Some(precio_final)
    }

    fn buscar_venta(&self, venta: &Venta) -> Option<&Venta> {
        self.ventas.iter().find(|v| *v == venta)
    }

    // ➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
    // permita visualizar las ventas totales por categoría de producto y otro por vendedor.
    fn reporte_ventas_categoria(&self) -> HashMap<String, f64> {
        let mut reporte: HashMap<String, f64> = HashMap::new();
        for v in &self.ventas {}
        reporte
    }

    fn reporte_ventas_vendedor(&self) -> HashMap<String, f64> {
        let mut reporte: HashMap<String, f64> = HashMap::new();
        for v in &self.ventas {}
        reporte
    }
}

/*
Se requiere implementar un sistema de ventas de productos.
De cada producto se conoce el nombre, una categoría y un precio base, y algunos productos pueden tener
descuentos aplicables dependiendo de la categoría. Además, se debe registrar al vendedor
que realizó la venta y al cliente. De ellos se conoce nombre, apellido, dirección, dni y del
vendedor nro de legajo, antigüedad y salario.
Los clientes pueden tener un beneficio de
descuento si tienen suscripción al newsletter, de ser así se tiene el correo electrónico del
mismo.
El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
Los medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia
bancaria y efectivo.

Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:

➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
productos con sus cantidades.

➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
calcularlo tenga en cuenta que pueden haber determinados productos de alguna
categoría donde debería aplicarse un descuento. Tanto la categoría como el
porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
aplicar un porcentaje de descuento general si el cliente tiene suscripción al
newsletter.

➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
permita visualizar las ventas totales por categoría de producto y otro por vendedor.

*/

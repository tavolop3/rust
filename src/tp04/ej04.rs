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

#[derive(Debug, Clone)]
struct Reporte {
    nombre: String,
    cantidad_ventas: u32,
}

impl Reporte {
    fn new(nombre: String, cantidad_ventas: u32) -> Self {
        Reporte {
            nombre,
            cantidad_ventas,
        }
    }
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
        registros: Vec<Registro>,
    ) -> Venta {
        let venta = Venta {
            vendedor,
            cliente,
            medio_pago,
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
    fn reporte_ventas_categoria(&self, categoria: &str) -> Reporte {
        let mut cantidad_ventas: u32 = 0;
        for v in &self.ventas {
            if v.registros
                .iter()
                .any(|r| r.producto.nombre_categoria == categoria)
            {
                cantidad_ventas += 1;
            }
        }

        Reporte::new(categoria.to_string(), cantidad_ventas)
    }

    fn reporte_ventas_vendedor(&self, nombre_vendedor: &str) -> Reporte {
        let cantidad_ventas = self
            .ventas
            .iter()
            .filter(|v| v.vendedor.nombre == nombre_vendedor)
            .count() as u32;

        Reporte::new(nombre_vendedor.to_string(), cantidad_ventas)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestData {
        sistema: Sistema,
        productos: Vec<Producto>,
        vendedor: Persona,
        cliente: Persona,
        fecha: Fecha,
    }

    fn setup() -> TestData {
        let mut sistema = Sistema::new();

        sistema.categorias.insert("Electrónica".to_string(), 20);
        sistema.categorias.insert("Ropa".to_string(), 10);

        let prod1 = Producto {
            nombre: "Celular".to_string(),
            nombre_categoria: "Electrónica".to_string(),
            precio_base: 1000.0,
        };
        let prod2 = Producto {
            nombre: "Camiseta".to_string(),
            nombre_categoria: "Ropa".to_string(),
            precio_base: 50.0,
        };

        let vendedor = Persona {
            nombre: "Juan".to_string(),
            apellido: "Pérez".to_string(),
            direccion: "Calle 123".to_string(),
            dni: 12345678,
            legajo: 1,
            antiguedad: 5,
            salario: 2000.0,
            email_newsletter: None,
        };
        let cliente = Persona {
            nombre: "Ana".to_string(),
            apellido: "Gómez".to_string(),
            direccion: "Av 456".to_string(),
            dni: 87654321,
            legajo: 0,
            antiguedad: 0,
            salario: 0.0,
            email_newsletter: Some("ana@example.com".to_string()),
        };

        sistema.crear_venta(
            Fecha {
                dia: 1,
                mes: 5,
                año: 2025,
            },
            cliente.clone(),
            vendedor.clone(),
            MedioPago::Efectivo,
            vec![Registro {
                producto: prod1.clone(),
                cantidad: 2,
            }],
        );
        sistema.crear_venta(
            Fecha {
                dia: 2,
                mes: 5,
                año: 2025,
            },
            cliente.clone(),
            vendedor.clone(),
            MedioPago::TarjetaCredito,
            vec![
                Registro {
                    producto: prod1.clone(),
                    cantidad: 1,
                },
                Registro {
                    producto: prod2.clone(),
                    cantidad: 3,
                },
            ],
        );

        TestData {
            sistema,
            productos: vec![prod1, prod2],
            vendedor,
            cliente,
            fecha: Fecha {
                dia: 1,
                mes: 5,
                año: 2025,
            },
        }
    }

    #[test]
    fn test_crear_venta() {
        let mut data = setup();
        let registros = vec![Registro {
            producto: data.productos[0].clone(),
            cantidad: 1,
        }];
        let venta = data.sistema.crear_venta(
            data.fecha.clone(),
            data.cliente.clone(),
            data.vendedor.clone(),
            MedioPago::Transferencia,
            registros.clone(),
        );

        assert_eq!(data.sistema.ventas.len(), 3);
        assert_eq!(venta.cliente, data.cliente);
        assert_eq!(venta.vendedor, data.vendedor);
        assert_eq!(venta.medio_pago, MedioPago::Transferencia);
        assert_eq!(venta.registros, registros);
    }

    #[test]
    fn test_calcular_precio_final() {
        let mut data = setup();
        let venta = &data.sistema.ventas[0].clone(); // 2 celulares (20% descuento)
        let precio = data.sistema.calcular_precio_final(venta).unwrap();
        // 2 * 1000 * 0.8 * 0.9 = 1440
        assert_eq!(precio, 1440.0);

        let venta = &data.sistema.ventas[1].clone(); // 1 celular + 3 camisetas
        let precio = data.sistema.calcular_precio_final(venta).unwrap();
        // (1 * 1000 * 0.8) + (3 * 50 * 0.9) = 800 + 135 = 935 * 0.9 = 841.5
        assert_eq!(precio, 841.5);
    }

    #[test]
    fn test_calcular_precio_final_venta_inexistente() {
        let mut data = setup();
        let venta = Venta {
            vendedor: data.vendedor.clone(),
            cliente: data.cliente.clone(),
            medio_pago: MedioPago::Efectivo,
            fecha: Fecha {
                dia: 3,
                mes: 5,
                año: 2025,
            },
            registros: vec![],
        };
        let precio = data.sistema.calcular_precio_final(&venta);
        assert!(precio.is_none());
    }

    #[test]
    fn test_reporte_ventas_categoria() {
        let data = setup();
        let reporte_electronica = data.sistema.reporte_ventas_categoria("Electrónica");
        let reporte_ropa = data.sistema.reporte_ventas_categoria("Ropa");
        let reporte_inexistente = data.sistema.reporte_ventas_categoria("Hogar");

        assert_eq!(reporte_electronica.cantidad_ventas, 2);
        assert_eq!(reporte_electronica.nombre, "Electrónica");
        assert_eq!(reporte_ropa.cantidad_ventas, 1);
        assert_eq!(reporte_ropa.nombre, "Ropa");
        assert_eq!(reporte_inexistente.cantidad_ventas, 0);
        assert_eq!(reporte_inexistente.nombre, "Hogar");
    }

    #[test]
    fn test_reporte_ventas_vendedor() {
        let data = setup();
        let reporte_juan = data.sistema.reporte_ventas_vendedor("Juan");
        let reporte_inexistente = data.sistema.reporte_ventas_vendedor("María");

        assert_eq!(reporte_juan.cantidad_ventas, 2);
        assert_eq!(reporte_juan.nombre, "Juan");
        assert_eq!(reporte_inexistente.cantidad_ventas, 0);
        assert_eq!(reporte_inexistente.nombre, "María");
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

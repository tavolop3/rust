#[allow(dead_code)]
pub struct Producto {
    precio:f64,
    id:u64,
    nombre: String,
}

impl Producto {
    pub fn new(nombre:&str, precio:f64, id:u64) -> Self {
        Producto {
            nombre: nombre.to_string(),
            precio: precio,
            id: id,
        }
    }

    pub fn calcular_impuestos(&self, impuestos:f64) -> f64 {
        self.precio * impuestos/100.0
    }
    
    pub fn aplicar_descuento(&self, descuento:f64) -> f64 {
        self.precio - (self.precio * descuento/100.0)
    }

    pub fn calcular_precio_total(&self, impuestos:Option<f64>, descuento:Option<f64>) -> f64 {
        let mut tot = self.precio;

        if let Some(d) = descuento {
            tot = self.aplicar_descuento(d);
        } else {
            tot = self.precio;
        }

        if let Some(i) = impuestos {
            tot += self.calcular_impuestos(i);
        }

        tot
    }
}

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

    // TODO: los parametros de esto tienen que ser opcionales
    pub fn calcular_precio_total(&self, impuestos:f64, descuento:f64) -> f64 {
        self.aplicar_descuento(descuento) + self.calcular_impuestos(impuestos)
    }
}

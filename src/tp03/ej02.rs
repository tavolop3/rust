pub struct Rectangulo {
    longitud:f64,
    ancho:f64,
}

impl Rectangulo {
    pub fn new(longitud:f64, ancho:f64) -> Self {
        Rectangulo { longitud, ancho }
    }

    pub fn calcular_area(&self) -> f64 {
        self.longitud * self.ancho
    }

    pub fn calcular_perimetro(&self) -> f64 {
        (self.longitud + self.ancho) * 2.0
    }

    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho 
    }
}

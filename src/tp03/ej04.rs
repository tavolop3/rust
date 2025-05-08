pub struct Triangulo(f32, f32, f32);

#[derive(Debug, PartialEq)]
pub enum TipoTriangulo {
    EQUILATERO,
    ISOCELES,
    ESCALENO,
}

impl Triangulo {
    pub fn new(l1: f32, l2: f32, l3: f32) -> Self {
        Triangulo(l1, l2, l3)
    }

    pub fn determinar_tipo(&self) -> TipoTriangulo {
        if self.0 == self.1 {
            if self.1 == self.2 {
                TipoTriangulo::EQUILATERO
            } else {
                TipoTriangulo::ISOCELES
            }
        } else {
            TipoTriangulo::ESCALENO
        }
    }

    //formula de Herón
    pub fn calcular_area(&self) -> f32 {
        let s = (self.0 + self.1 + self.2) / 2.0;
        (s * (s - self.0) * (s - self.1) * (s - self.2)).sqrt()
    }

    pub fn calcular_perimetro(&self) -> f32 {
        self.0 + self.1 + self.2
    }
}

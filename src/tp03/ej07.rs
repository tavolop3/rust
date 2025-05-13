#![allow(dead_code, unused_variables)]

pub struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u16,
    autos: Vec<Auto>,
}

#[derive(Debug, Clone)]
pub struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f64,
    color: Color,
}

#[derive(Debug, Clone)]
pub enum Color {
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad: u16) -> Self {
        let autos: Vec<Auto> = vec![];
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos,
        }
    }

    pub fn agregar_auto(&mut self, auto: &Auto) -> bool {
        if self.capacidad == self.autos.len() as u16 {
            return false;
        }

        self.autos.push(auto.clone());
        true
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) {
        for i in 0..self.autos.len() {
            if self.autos[i].comparar(auto) {
                self.autos.swap_remove(i);
                return;
            }
        }
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<Auto> {
        let mut resul: Option<Auto>;

        for a in &self.autos {
            if a.comparar(auto) {
                return Some(a.clone());
            }
        }
        None
    }
}

impl Auto {
    pub fn new(marca: String, modelo: String, año: u32, precio_bruto: f64, color: Color) -> Self {
        Auto {
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }

    fn comparar(&self, a2: &Auto) -> bool {
        self.marca == a2.marca && self.modelo == a2.modelo
    }

    pub fn get_info(&self) -> String {
        format!("Marca: {}, Modelo: {}", self.marca, self.modelo)
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut tasa = match self.color {
            Color::ROJO | Color::AMARILLO | Color::AZUL => 0.25,
            _ => -0.10,
        };

        if self.marca == "BMW" {
            tasa += 0.15;
        }

        if self.año < 2000 {
            tasa -= 0.05;
        }

        self.precio_bruto * (1.0 + tasa)
    }
}

pub struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u64,
    cola_atencion: Vec<Atencion>,
}

pub struct Mascota {
    nombre: String,
    edad: u8,
    tipo: TipoAnimal,
    dueño: Cliente,
}

pub struct Cliente {
    nombre: String,
    direccion: String,
    telefono: String,
}

pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

use super::ej03::Fecha;
pub struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha: Fecha,
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u64) -> Self {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_atencion: Vec::new(),
        }
    }
}

pub struct Estudiante {
    nombre:String,
    id:u64,
    examenes:Vec<Examen>,
}

pub struct Examen {
    nombre:String,
    nota:u8,
}

impl Examen {
    pub fn new(nombre:&str, nota:u8) {
        Examen {
            nombre.to_string(),
            nota
        }
    }
}

impl Estudiante {
    pub fn new(nombre:&str, id:u64, examenes:Vec<Examen>) {
        Estudiante {
            nombre.to_string(),
            id,
            examenes
        }
    }

    pub fn obtener_promedio(&self) -> Examen {
        
    }

    pub fn obtener_calificacion_mas_alta(&self) -> Examen {

    }
}

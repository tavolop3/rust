pub struct Estudiante {
    nombre: String,
    id: u64,
    examenes: Vec<Examen>,
}

pub struct Examen {
    nombre: String,
    nota: f32,
}

impl Examen {
    pub fn new(nombre: &str, nota: f32) -> Self {
        Examen {
            nombre: nombre.to_string(),
            nota,
        }
    }
}

impl Estudiante {
    pub fn new(nombre: &str, id: u64, examenes: Vec<Examen>) -> Self {
        Estudiante {
            nombre: nombre.to_string(),
            id,
            examenes,
        }
    }

    pub fn obtener_promedio(&self) -> f32 {
        if self.examenes.is_empty() {
            return 0.0;
        }

        let mut tot = 0.0;
        for e in &self.examenes {
            tot += e.nota
        }

        tot / self.examenes.len() as f32
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f32 {
        if self.examenes.is_empty() {
            return 0.0;
        }

        let mut max = self.examenes[0].nota;
        for e in &self.examenes {
            if e.nota > max {
                max = e.nota;
            }
        }

        max
    }

    pub fn obtener_calificacion_mas_baja(&self) -> f32 {
        if self.examenes.is_empty() {
            return 0.0;
        }

        let mut min = self.examenes[0].nota;
        for e in &self.examenes {
            if e.nota < min {
                min = e.nota;
            }
        }

        min
    }
}

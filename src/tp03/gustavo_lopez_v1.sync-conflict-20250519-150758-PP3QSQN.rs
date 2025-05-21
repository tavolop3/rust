#![allow(dead_code, unused_variables)]

pub struct Estudiante {
    nombre: String,
    id: u64,
    examenes: Vec<Examen>,
}

#[derive(Debug, Clone)]
pub struct Examen {
    nombre: String,
    nota: f32,
}

pub struct Informe {
    pub nombre_estudiante: String,
    pub cant_rendidos: u8,
    pub promedio: f32,
    pub nota_mas_alta: Examen,
    pub nota_mas_baja: Examen,
}

impl Examen {
    pub fn new(nombre: &str, nota: f32) -> Self {
        Examen {
            nombre: nombre.to_string(),
            nota,
        }
    }

    pub fn get_nota(&self) -> f32 {
        self.nota
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

    pub fn obtener_calificacion_mas_alta(&self) -> Option<Examen> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut examen = self.examenes[0].clone();
        let mut max = examen.nota;
        for e in &self.examenes {
            if e.nota > max {
                max = e.nota;
                examen = e.clone();
            }
        }

        Some(examen)
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<Examen> {
        if self.examenes.is_empty() {
            return None;
        }

        let mut examen = self.examenes[0].clone();
        let mut min = examen.nota;
        for e in &self.examenes {
            if e.nota < min {
                min = e.nota;
                examen = e.clone();
            }
        }

        Some(examen)
    }

    pub fn generar_informe(&self) -> Option<Informe> {
        if self.examenes.is_empty() {
            return None;
        }

        let cant_rendidos = self.examenes.len() as u8;
        let nombre_estudiante = self.nombre.clone();
        let promedio = self.obtener_promedio();
        let nota_mas_alta = self.obtener_calificacion_mas_alta();
        let nota_mas_baja = self.obtener_calificacion_mas_baja();

        let informe = Informe::new(
            nombre_estudiante,
            cant_rendidos,
            promedio,
            nota_mas_alta?,
            nota_mas_baja?,
        );
        Some(informe)
    }
}

impl Informe {
    pub fn new(
        nombre_estudiante: String,
        cant_rendidos: u8,
        promedio: f32,
        nota_mas_alta: Examen,
        nota_mas_baja: Examen,
    ) -> Self {
        Informe {
            nombre_estudiante,
            cant_rendidos,
            promedio,
            nota_mas_alta,
            nota_mas_baja,
        }
    }
}

//esto lo tenia en un archivo general de tests
//deberia separarlo? si es asi lo separo en mas funciones
//yo directamente testee todo en una sola función
//tambien me gustaria hacer un getter por campo del informe, no hacerlo publico
#[test]
fn test_ej06_examenes() {
    use crate::tp03::ej06::Estudiante;
    use crate::tp03::ej06::Examen;

    let ex1 = Examen::new("Rust", 8.5);
    let ex2 = Examen::new("OO2", 7.0);
    let ex3 = Examen::new("SO", 9.0);
    let est = Estudiante::new("Alan Turing", 1, vec![ex1, ex2, ex3]);
    assert_eq!(est.obtener_promedio(), (8.5 + 7.0 + 9.0) / 3.0);
    assert_eq!(est.obtener_calificacion_mas_alta().unwrap().get_nota(), 9.0);
    assert_eq!(est.obtener_calificacion_mas_baja().unwrap().get_nota(), 7.0);
    let est_vacio = Estudiante::new("Ana Gómez", 67890, vec![]);

    assert_eq!(est_vacio.obtener_promedio(), 0.0);
    assert!(est_vacio.obtener_calificacion_mas_alta().is_none());
    assert!(est_vacio.obtener_calificacion_mas_baja().is_none());

    let informe_lleno = est.generar_informe();
    assert!(informe_lleno.is_some());
    assert!(est_vacio.generar_informe().is_none());

    // acá quiero testear cada campo
    // assert_eq!(informe_lleno.unwrap().nombre_estudiante, est.);
}

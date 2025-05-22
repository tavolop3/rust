#![allow(dead_code, unused_variables)]
use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

impl<'a> Persona<'a> {
    pub fn new(
        nombre: &'a str,
        apellido: &'a str,
        direccion: &'a str,
        ciudad: &'a str,
        salario: f64,
        edad: u8,
    ) -> Self {
        Persona {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}

fn salario_mayor_a<'a>(personas: &Vec<Persona<'a>>, salario: f64) -> Vec<Persona<'a>> {
    personas
        .iter()
        .filter(|&p| p.salario > salario)
        .cloned()
        .collect()
}

fn edad_vive_en<'a>(personas: &Vec<Persona<'a>>, edad: u8, ciudad: &'a str) -> Vec<Persona<'a>> {
    personas
        .iter()
        .filter(|&p| p.edad > edad && p.ciudad == ciudad)
        .cloned()
        .collect()
}

fn viven_en<'a>(personas: &Vec<Persona<'a>>, ciudad: &'a str) -> bool {
    personas.iter().all(|p| p.ciudad == ciudad)
}

fn vive_almenos_uno<'a>(personas: &Vec<Persona<'a>>, ciudad: &'a str) -> bool {
    personas.iter().any(|p| p.ciudad == ciudad)
}

fn tiene_persona<'a>(personas: &Vec<Persona<'a>>, persona: &Persona<'a>) -> bool {
    personas.iter().any(|p| p == persona)
}

fn devolver_edades(personas: &Vec<Persona>) -> Vec<u8> {
    personas.iter().map(|p| p.edad).collect()
}

// g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
// salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
// categoría desempatar por la edad más grande.
struct Resultado<'a> {
    p_max_salario: Persona<'a>,
    p_min_salario: Persona<'a>,
}

// TODO: Ver bien esto
fn devolver_salarios_limite<'a>(personas: &Vec<Persona<'a>>) -> Resultado<'a> {
    let p_max = personas.iter().max_by(|p1, p2| {
        p1.salario
            .partial_cmp(&p2.salario)
            .unwrap_or(Ordering::Equal)
            .then_with(|| p1.edad.cmp(&p2.edad))
    });

    let p_min = personas.iter().min_by(|p1, p2| {
        p1.salario
            .partial_cmp(&p2.salario)
            .unwrap_or(Ordering::Equal)
            .then_with(|| p2.edad.cmp(&p1.edad))
    });

    Resultado {
        p_max_salario: p_max.unwrap().clone(),
        p_min_salario: p_min.unwrap().clone(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn data<'a>() -> Vec<Persona<'a>> {
        let p1: Persona = Persona::new("tao", "lop", "calle 1", "la plata", 10_000_000.0, 23);
        let p2: Persona = Persona::new("opa", "lopa", "calle 2", "la plata", 1_000_000.0, 19);
        let p3: Persona = Persona::new("aoa", "a", "calle 5", "capital", 1_000_000.0, 40);
        vec![p1, p2, p3]
    }

    #[test]
    fn test_ej02_a_salario() {
        let personas = data();
        assert!(salario_mayor_a(&personas, 1_000_000.0).len() == 1);
        assert!(salario_mayor_a(&personas, 10_000_000.0).is_empty());
    }

    #[test]
    fn test_ej02_b_edad_ciudad() {
        let personas = data();
        assert!(edad_vive_en(&personas, 18, "la plata").len() == 2);
        assert!(edad_vive_en(&personas, 35, "capital").len() == 1);
    }

    #[test]
    fn test_ej02_c_ciudad() {
        let mut personas = data();
        assert!(!viven_en(&personas, "la plata"));
        personas.remove(2);
        assert!(viven_en(&personas, "la plata"));
    }

    #[test]
    fn test_ej02_d_vive_almenos_uno() {
        let personas = data();
        assert!(vive_almenos_uno(&personas, "la plata"));
        assert!(vive_almenos_uno(&personas, "capital"));
        assert!(!vive_almenos_uno(&personas, "mar del plata"));
    }

    #[test]
    fn test_ej02_e_existe_persona() {
        let personas = data();
        let persona = &personas[0];
        assert!(tiene_persona(&personas, persona));
        let persona = &Persona::new("no", "existe", "en", "el vec", 33.0, 33);
        assert!(!tiene_persona(&personas, persona));
    }

    #[test]
    fn test_ej02_f_devolver_edades() {
        let personas = data();
        let resul = devolver_edades(&personas);
        assert!(resul.len() == 3);
        assert!(resul[0] == 23);
        assert!(resul[1] == 19);
        assert!(resul[2] == 40);
    }

    #[test]
    fn test_ej02_g_salarios_limite() {
        let mut personas = data();
        let resul = devolver_salarios_limite(&personas);
        assert!(resul.p_min_salario == personas[2]);
        let persona_nueva: Persona =
            Persona::new("e", "e", "calle 50", "capital", 10_000_000.0, 50);
        personas.push(persona_nueva);
        let resul = devolver_salarios_limite(&personas);
        assert!(resul.p_max_salario == personas[3]);
    }
}

/*
https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision

2- Dado el siguiente struct:
struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.

b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad.

c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario.

d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario.

e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la persona existe en el arreglo, false caso contrario

f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.

g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.

Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure.

*/

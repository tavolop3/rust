#![allow(dead_code, unused_variables)]

#[derive(Clone)]
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

// TODO: no entender pq no hace lifetime elision pero bueno
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

#[cfg(test)]
mod test {
    use super::*;

    fn data<'a>() -> Vec<Persona<'a>> {
        let p1: Persona = Persona::new("tao", "lop", "calle 1", "la plata", 10_000_000.0, 23);
        let p2: Persona = Persona::new("opa", "lopa", "calle 2", "la plata", 1_000_000.0, 19);
        vec![p1, p2]
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
        assert!(edad_vive_en(&personas, 40, "capital").is_empty());
    }

    #[test]
    fn test_ej02_c_ciudad() {
        let personas = data();
        assert!(viven_en(&personas, "la plata"));
        assert!(!viven_en(&personas, "capital"));
    }

    #[test]
    fn test_ej02_d_vive_almenos_uno() {
        let personas = data();
        assert!(vive_almenos_uno(&personas, "la plata"));
        assert!(!vive_almenos_uno(&personas, "capital"));
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

e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario

f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.

g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.

Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure.

*/

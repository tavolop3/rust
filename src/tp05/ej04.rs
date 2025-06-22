#![allow(dead_code, unused_variables)]
use crate::tp03::ej03::Fecha;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub struct Biblioteca {
    nombre: String,
    direccion: String,
    prestamos: Vec<Prestamo>,
    disponibles: Vec<RegistroDisponible>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistroDisponible {
    libro: Libro,
    cant_disponibles: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    cant_paginas: u16,
    genero: Genero,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Estado {
    Devuelto,
    EnPrestamo,
}

impl Biblioteca {
    pub fn new(nombre: String, direccion: String) -> Self {
        Biblioteca {
            nombre,
            direccion,
            prestamos: vec![],
            disponibles: vec![],
        }
    }

    pub fn prestar(&mut self, cli: &Cliente, lib: &Libro, fecha_vencimiento: &Fecha) -> bool {
        if self.cant_prestamos_cli(cli) >= 5 || self.cant_disponibles(lib) == 0 {
            return false;
        }

        let p = Prestamo::new(lib, cli, fecha_vencimiento);
        self.prestamos.push(p);
        self.decrementar_disponibilidad(lib);
        self.persistir_prestamos();

        true
    }

    pub fn prestamos_a_vencer(&self, dias: i64) -> Vec<Prestamo> {
        let mut pres: Vec<Prestamo> = vec![];
        let mut f = Fecha::fecha_actual();
        f.sumar_dias(dias);
        for p in &self.prestamos {
            if !p.fecha_vencimiento.es_mayor(&f) {
                pres.push(p.clone());
            }
        }
        pres
    }

    pub fn prestamos_vencidos(&self) -> Vec<Prestamo> {
        let mut pres: Vec<Prestamo> = vec![];
        let fecha_actual = Fecha::fecha_actual();
        for p in &self.prestamos {
            if !p.fecha_vencimiento.es_mayor(&fecha_actual) {
                pres.push(p.clone());
            }
        }
        pres
    }

    pub fn buscar_prestamo(&self, lib: &Libro, cli: &Cliente) -> Option<Prestamo> {
        for p in &self.prestamos {
            if p.libro.igual(lib) && p.cliente.igual(cli) {
                return Some(p.clone());
            }
        }
        None
    }

    pub fn devolver_libro(&mut self, lib: &Libro, cli: &Cliente) -> bool {
        let mut encontrado = false;
        for p in &mut self.prestamos {
            if p.libro.igual(lib) && p.cliente.igual(cli) {
                p.estado = Estado::Devuelto;
                p.fecha_devolucion = Some(Fecha::fecha_actual());
                encontrado = true;
            }
        }
        if encontrado {
            self.incrementar_disponibilidad(lib);
            self.persistir_prestamos();
            return true;
        }
        false
    }

    pub fn incrementar_disponibilidad(&mut self, lib: &Libro) {
        for d in &mut self.disponibles {
            if d.libro.igual(lib) {
                d.cant_disponibles += 1;
                return;
            }
        }
        self.persistir_libros();
    }

    pub fn decrementar_disponibilidad(&mut self, lib: &Libro) {
        for d in &mut self.disponibles {
            if d.libro.igual(lib) {
                if d.cant_disponibles > 0 {
                    d.cant_disponibles -= 1;
                }
                return;
            }
        }
        self.persistir_libros();
    }

    pub fn cant_prestamos_cli(&self, cli: &Cliente) -> u8 {
        let mut cant_prestamos = 0;
        for p in &self.prestamos {
            if p.cliente.igual(cli) && p.estado.igual(&Estado::EnPrestamo) {
                cant_prestamos += 1;
            }
        }
        cant_prestamos
    }

    pub fn cant_disponibles(&self, lib: &Libro) -> u8 {
        let mut disponibles: u8 = 0;
        for d in &self.disponibles {
            if d.libro.igual(lib) {
                disponibles = d.cant_disponibles;
                break;
            }
        }
        disponibles
    }

    pub fn agregar_libro(&mut self, libro: &Libro, cant: u8) {
        self.disponibles
            .push(RegistroDisponible::new(libro.clone(), cant));
        self.persistir_libros();
    }

    pub fn quitar_prestamo(&mut self, cli: &Cliente, lib: &Libro) {
        self.prestamos
            .retain(|prestamo| !(prestamo.libro.igual(lib) && prestamo.cliente.igual(cli)));
        self.persistir_prestamos();
    }

    pub fn persistir_libros(&self) {
        let mut f = File::create("src/tp05/registros/ej04/libros_disponibles.json").unwrap();
        let registros_serializado = serde_json::to_string_pretty(&self.disponibles).unwrap();
        f.write_all(registros_serializado.as_bytes()).unwrap();
    }

    pub fn persistir_prestamos(&self) {
        let mut f = File::create("src/tp05/registros/ej04/prestamos.json").unwrap();
        let registros_serializado = serde_json::to_string_pretty(&self.prestamos).unwrap();
        f.write_all(registros_serializado.as_bytes()).unwrap();
    }
}

impl Libro {
    pub fn new(
        isbn: String,
        titulo: String,
        autor: String,
        cant_paginas: u16,
        genero: Genero,
    ) -> Self {
        Libro {
            isbn,
            titulo,
            autor,
            cant_paginas,
            genero,
        }
    }

    pub fn igual(&self, l: &Libro) -> bool {
        self.isbn == l.isbn
    }
}

impl Cliente {
    pub fn new(nombre: String, telefono: String, email: String) -> Self {
        Cliente {
            nombre,
            telefono,
            email,
        }
    }

    pub fn igual(&self, c: &Cliente) -> bool {
        self.email == c.email
    }
}

impl RegistroDisponible {
    pub fn new(libro: Libro, cant_disponibles: u8) -> Self {
        RegistroDisponible {
            libro,
            cant_disponibles,
        }
    }
}

impl Prestamo {
    pub fn new(libro: &Libro, cliente: &Cliente, fecha_vencimiento: &Fecha) -> Self {
        Prestamo {
            libro: libro.clone(),
            cliente: cliente.clone(),
            fecha_vencimiento: fecha_vencimiento.clone(),
            fecha_devolucion: None,
            estado: Estado::EnPrestamo,
        }
    }
}

impl Estado {
    pub fn igual(&self, estado: &Estado) -> bool {
        self.a_str() == estado.a_str()
    }

    pub fn a_str(&self) -> String {
        match self {
            Estado::Devuelto => String::from("Devuelto"),
            Estado::EnPrestamo => String::from("EnPrestamo"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestData {
        biblioteca: Biblioteca,
        libro1: Libro,
        libro2: Libro,
        cliente1: Cliente,
        cliente2: Cliente,
        fecha_vencimiento: Fecha,
        fecha_pasada: Fecha,
    }

    fn setup() -> TestData {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca Informatica".to_string(),
            "Calle 123".to_string(),
        );
        let libro1 = Libro::new(
            "12345".to_string(),
            "Un libro".to_string(),
            "Un autor1".to_string(),
            400,
            Genero::Novela,
        );
        let libro2 = Libro::new(
            "67890".to_string(),
            "La biblia de C".to_string(),
            "Brian Cranston".to_string(),
            120,
            Genero::Tecnico,
        );
        let cliente1 = Cliente::new(
            "Juan Perez".to_string(),
            "123456789".to_string(),
            "juan@email.com".to_string(),
        );
        let cliente2 = Cliente::new(
            "Pepito Lopez".to_string(),
            "987654321".to_string(),
            "pepito@email.com".to_string(),
        );
        let mut fecha_vencimiento = Fecha::fecha_actual();
        fecha_vencimiento.sumar_dias(7);
        let mut fecha_pasada = Fecha::fecha_actual();
        fecha_pasada.restar_dias(1);

        biblioteca.agregar_libro(&libro1, 5);
        biblioteca.agregar_libro(&libro2, 4);

        TestData {
            biblioteca,
            libro1,
            libro2,
            cliente1,
            cliente2,
            fecha_vencimiento,
            fecha_pasada,
        }
    }

    #[test]
    fn test_biblioteca_new() {
        let biblioteca = Biblioteca::new("Test Biblioteca".to_string(), "Avenida 456".to_string());
        assert_eq!(biblioteca.nombre, "Test Biblioteca");
        assert_eq!(biblioteca.direccion, "Avenida 456");
        assert!(biblioteca.prestamos.is_empty());
        assert!(biblioteca.disponibles.is_empty());
    }

    #[test]
    fn test_libro_new_and_igual() {
        let libro1 = Libro::new(
            "12345".to_string(),
            "Un libro".to_string(),
            "Un autor".to_string(),
            400,
            Genero::Novela,
        );
        let libro2 = Libro::new(
            "12345".to_string(),
            "Otro libro".to_string(),
            "Otro autor".to_string(),
            200,
            Genero::Infantil,
        );
        let libro3 = Libro::new(
            "67890".to_string(),
            "Un libro".to_string(),
            "Un autor".to_string(),
            400,
            Genero::Novela,
        );
        assert!(libro1.igual(&libro2)); // Mismo ISBN
        assert!(!libro1.igual(&libro3)); // Distinto ISBN
    }

    #[test]
    fn test_cliente_new_and_igual() {
        let cliente1 = Cliente::new(
            "Juan".to_string(),
            "123456789".to_string(),
            "juan@email.com".to_string(),
        );
        let cliente2 = Cliente::new(
            "Pedro".to_string(),
            "987654321".to_string(),
            "juan@email.com".to_string(),
        );
        let cliente3 = Cliente::new(
            "Juan".to_string(),
            "123456789".to_string(),
            "pedro@email.com".to_string(),
        );
        assert!(cliente1.igual(&cliente2)); // Mismo email
        assert!(!cliente1.igual(&cliente3)); // Distinto email
    }

    #[test]
    fn test_registro_disponible_new() {
        let libro = Libro::new(
            "12345".to_string(),
            "Un libro".to_string(),
            "Un autor".to_string(),
            400,
            Genero::Novela,
        );
        let registro = RegistroDisponible::new(libro.clone(), 3);
        assert!(registro.libro.igual(&libro));
        assert_eq!(registro.cant_disponibles, 3);
    }

    #[test]
    fn test_prestamo_new() {
        let data = setup();
        let prestamo = Prestamo::new(&data.libro1, &data.cliente1, &data.fecha_vencimiento);
        assert!(prestamo.libro.igual(&data.libro1));
        assert!(prestamo.cliente.igual(&data.cliente1));
        assert_eq!(prestamo.fecha_devolucion, None);
        assert!(prestamo.estado.igual(&Estado::EnPrestamo));
    }

    #[test]
    fn test_estado_igual_and_a_str() {
        assert!(Estado::Devuelto.igual(&Estado::Devuelto));
        assert!(Estado::EnPrestamo.igual(&Estado::EnPrestamo));
        assert!(!Estado::Devuelto.igual(&Estado::EnPrestamo));
        assert_eq!(Estado::Devuelto.a_str(), "Devuelto");
        assert_eq!(Estado::EnPrestamo.a_str(), "EnPrestamo");
    }

    #[test]
    fn test_agregar_libro() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        let libro3 = Libro::new(
            "99999".to_string(),
            "Nuevo libro".to_string(),
            "Nuevo autor".to_string(),
            300,
            Genero::Otros,
        );
        biblioteca.agregar_libro(&libro3, 2);
        assert_eq!(biblioteca.cant_disponibles(&libro3), 2);
    }

    #[test]
    fn test_cant_disponibles() {
        let data = setup();
        let biblioteca = data.biblioteca;
        assert_eq!(biblioteca.cant_disponibles(&data.libro1), 5);
        assert_eq!(biblioteca.cant_disponibles(&data.libro2), 4);
        let libro_no_existe = Libro::new(
            "00000".to_string(),
            "No existe".to_string(),
            "Nadie".to_string(),
            100,
            Genero::Infantil,
        );
        assert_eq!(biblioteca.cant_disponibles(&libro_no_existe), 0);
    }

    #[test]
    fn test_incrementar_decrementar_disponibilidad() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.decrementar_disponibilidad(&data.libro1);
        assert_eq!(biblioteca.cant_disponibles(&data.libro1), 4);
        biblioteca.incrementar_disponibilidad(&data.libro1);
        assert_eq!(biblioteca.cant_disponibles(&data.libro1), 5);
        let libro_no_existe = Libro::new(
            "00000".to_string(),
            "No existe".to_string(),
            "Nadie".to_string(),
            100,
            Genero::Infantil,
        );
        biblioteca.incrementar_disponibilidad(&libro_no_existe); // No deberia paniquear
        biblioteca.decrementar_disponibilidad(&libro_no_existe); // No deberia paniquear
    }

    #[test]
    fn test_prestar_success() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        assert!(biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento));
        assert_eq!(biblioteca.cant_disponibles(&data.libro1), 4);
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 1);
    }

    #[test]
    fn test_prestar_fail_max_prestamos() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        // Alcanza el maximo de 5 prestamos
        for _ in 0..5 {
            biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento);
        }
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 5);
        assert!(!biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento));
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 5); // No cambia
    }

    #[test]
    fn test_prestar_fail_no_copies() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        // Deplete all copies of libro2
        for _ in 0..4 {
            biblioteca.decrementar_disponibilidad(&data.libro2);
        }
        assert_eq!(biblioteca.cant_disponibles(&data.libro2), 0);
        assert!(!biblioteca.prestar(&data.cliente1, &data.libro2, &data.fecha_vencimiento));
    }

    #[test]
    fn test_cant_prestamos_cli() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento);
        biblioteca.prestar(&data.cliente1, &data.libro2, &data.fecha_vencimiento);
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 2);
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente2), 0);

        biblioteca.devolver_libro(&data.libro1, &data.cliente1);
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 1);
    }

    #[test]
    fn test_buscar_prestamo() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento);
        assert!(
            biblioteca
                .buscar_prestamo(&data.libro1, &data.cliente1)
                .is_some()
        );
        assert!(
            biblioteca
                .buscar_prestamo(&data.libro1, &data.cliente2)
                .is_none()
        );
        assert!(
            biblioteca
                .buscar_prestamo(&data.libro2, &data.cliente1)
                .is_none()
        );
    }

    #[test]
    fn test_prestamos_a_vencer() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento);
        biblioteca.prestar(&data.cliente2, &data.libro2, &data.fecha_pasada);
        let prestamos = biblioteca.prestamos_a_vencer(10);
        assert_eq!(prestamos.len(), 2);
        let prestamos = biblioteca.prestamos_a_vencer(1);
        assert_eq!(prestamos.len(), 1);
    }

    #[test]
    fn test_prestamos_vencidos() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_pasada);
        biblioteca.prestar(&data.cliente2, &data.libro2, &data.fecha_vencimiento);
        let prestamos = biblioteca.prestamos_vencidos();
        assert_eq!(prestamos.len(), 1);
    }

    #[test]
    fn test_devolver_libro() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento);
        assert_eq!(biblioteca.cant_disponibles(&data.libro1), 4);
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 1);
        assert!(biblioteca.devolver_libro(&data.libro1, &data.cliente1));
        assert_eq!(biblioteca.cant_disponibles(&data.libro1), 5);
        assert_eq!(biblioteca.cant_prestamos_cli(&data.cliente1), 0);

        assert!(!biblioteca.devolver_libro(&data.libro2, &data.cliente1));
    }

    #[test]
    fn test_quitar_prestamo() {
        let data = setup();
        let mut biblioteca = data.biblioteca;
        biblioteca.prestar(&data.cliente1, &data.libro1, &data.fecha_vencimiento);
        assert_eq!(biblioteca.prestamos.len(), 1);
        biblioteca.quitar_prestamo(&data.cliente1, &data.libro1);
        assert_eq!(biblioteca.prestamos.len(), 0);

        biblioteca.quitar_prestamo(&data.cliente1, &data.libro2);
        assert_eq!(biblioteca.prestamos.len(), 0);
    }
}

/*
4- En base al ejercicio 10 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Tanto los libros con sus copias como la administración de préstamos se realizan
sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo
así. No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos para cumplir con este punto . Recuerde también que se debe seguir
manteniendo un coverage de al menos 90%.
*/

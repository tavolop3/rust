#![allow(dead_code, unused_variables)]
use crate::tp03::ej03::Fecha;

pub struct Biblioteca {
    nombre: String,
    direccion: String,
    prestamos: Vec<Prestamo>,
    disponibles: Vec<RegistroDisponible>,
}

#[derive(Clone)]
pub struct RegistroDisponible {
    libro: Libro,
    cant_disponibles: u8,
}

#[derive(Clone)]
pub struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    cant_paginas: u16,
    genero: Genero,
}

#[derive(Clone)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}

#[derive(Clone)]
pub struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}

#[derive(Clone)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Clone)]
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
        if self.cant_prestamos_cli(cli) >= 5 {
            return false;
        }
        if self.cant_disponibles(lib) == 0 {
            return false;
        }

        let p = Prestamo::new(lib, cli, fecha_vencimiento);
        self.prestamos.push(p);
        self.decrementar_disponibilidad(lib);

        true
    }

    pub fn prestamos_a_vencer(&self, dias: i64) -> Vec<Prestamo> {
        let mut pres: Vec<Prestamo> = vec![];
        for p in &self.prestamos {
            let mut f = p.fecha_vencimiento.clone();
            f.sumar_dias(dias);
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
    }

    pub fn quitar_prestamo(&mut self, cli: &Cliente, lib: &Libro) {
        let mut prestamo_i = 0;
        for i in 0..self.prestamos.len() {
            if self.prestamos[i].libro.igual(lib) && self.prestamos[i].cliente.igual(cli) {
                prestamo_i = i;
            }
        }
        self.prestamos.swap_remove(prestamo_i);
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

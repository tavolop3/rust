#![allow(dead_code, unused_variables)]
use crate::tp03::ej03::Fecha;

pub struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: Vec<Libro>,
    prestamos: Vec<Prestamo>,
}

#[derive(Clone)]
pub struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    cant_paginas: u16,
    genero: Genero,
    cant_disponibles: u8,
}

#[derive(Clone)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Fecha,
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

#[derive(Clone, PartialEq)]
pub enum Estado {
    Devuelto,
    EnPrestamo,
}

impl Biblioteca {
    pub fn new(nombre: String, direccion: String) -> Self {
        Biblioteca {
            nombre,
            direccion,
            libros: vec![],
            prestamos: vec![],
        }
    }

    pub fn cant_prestamos_cliente(&self, email: String) -> u8 {
        let mut cant = 0;
        for p in &self.prestamos {
            if p.cliente.email == email {
                cant += 1;
            }
        }
        cant
    }

    pub fn prestar(&mut self, cli: Cliente, lib: Libro) -> bool {
        let mut cant_prestamos = 0;
        for p in &self.prestamos {
            if p.cliente.igual(&cli) && p.estado == Estado::EnPrestamo {
                cant_prestamos += 1;
            }
        }

        if cant_prestamos >= 5 {
            return false;
        }

        let mut disponibles = 0;
        let mut lib_i = 0;
        for i in 0..self.libros.len() {
            if self.libros[i].igual(&lib) {
                disponibles = lib.cant_disponibles;
                lib_i = i;
                break;
            }
        }

        if disponibles == 0 {
            return false;
        }

        self.libros[lib_i].cant_disponibles -= 1;

        true
    }

    pub fn prestamos_a_vencer(&self, dias: u8) -> Vec<Prestamo> {
        let mut pres: Vec<Prestamo> = vec![];
        for p in &self.prestamos {
            let mut f = p.fecha_vencimiento.clone();
            f.sumar_dias(dias);
            if !p.fecha_vencimiento.es_mayor(f) {
                pres.push(p.clone());
            }
        }
        pres
    }

    // pub fn prestamos_vencidos(&self) -> Vec<Prestamo> {
    //     let mut pres: Vec<Prestamo> = vec![];
    //     for p in &self.prestamos {
    //         //TODO: hacer que fecha tenga un metodo de fecha de hoy y compararla con esa, ademas de
    //         //implementarlo con un crate
    //         if !p.fecha_vencimiento.es_mayor(Fecha::new())
    //     }
    // }

    pub fn buscar_prestamo(&self, lib: &Libro, cli: &Cliente) -> Option<Prestamo> {
        for p in &self.prestamos {
            if p.libro.igual(lib) && p.cliente.igual(cli) {
                return Some(p.clone());
            }
        }
        None
    }

    pub fn devolver_libro(&mut self, lib: &Libro, cli: &Cliente) -> bool {
        let prestamo = self.buscar_prestamo(lib, cli);
        if let Some(mut p) = prestamo {
            p.estado = Estado::Devuelto;
            // p.fecha_devolucion = Fecha::ahora();
            p.libro.cant_disponibles += 1;
            true
        } else {
            false
        }
    }
}

impl Libro {
    pub fn new(
        isbn: String,
        titulo: String,
        autor: String,
        cant_paginas: u16,
        genero: Genero,
        cant_disponibles: u8,
    ) -> Self {
        Libro {
            isbn,
            titulo,
            autor,
            cant_paginas,
            genero,
            cant_disponibles,
        }
    }

    pub fn get_cant_copias(&self) -> u8 {
        self.cant_disponibles
    }

    pub fn dec_cant_disponibles(&mut self) -> u8 {
        if self.cant_disponibles == 0 {
            return 0;
        }
        self.cant_disponibles -= 1;
        self.cant_disponibles
    }

    pub fn inc_cant_disponibles(&mut self) -> u8 {
        self.cant_disponibles += 1;
        self.cant_disponibles
    }

    pub fn igual(&self, l: &Libro) -> bool {
        self.isbn == l.isbn
    }
}

impl Cliente {
    pub fn igual(&self, c: &Cliente) -> bool {
        self.email == c.email
    }
}

#![allow(unused_variables, dead_code)]

use super::ej03::Fecha;
use ::std::collections::VecDeque;

pub struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u64,
    cola_atencion: VecDeque<Mascota>,
    registro_atencion: Vec<RegistroAtencion>,
}

#[derive(Debug, Clone)]
pub struct Mascota {
    nombre: String,
    edad: u8,
    tipo: TipoAnimal,
    dueño: Cliente,
}

#[derive(Debug, Clone)]
pub struct Cliente {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, Clone)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone)]
pub struct RegistroAtencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha: Fecha,
    pub proxima_fecha: Option<Fecha>,
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u64) -> Self {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            registro_atencion: Vec::new(),
        }
    }

    pub fn agregar_mascota(&mut self, mascota: &Mascota) {
        self.cola_atencion.push_back(mascota.clone());
    }

    pub fn agregar_mascota_urgente(&mut self, mascota: &Mascota) {
        self.cola_atencion.push_front(mascota.clone());
    }

    pub fn atender(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    pub fn eliminar_mascota(&mut self, m: &Mascota) -> bool {
        for i in 0..self.cola_atencion.len() {
            if self.cola_atencion[i].comparar(m) {
                self.cola_atencion.remove(i);
                return true;
            }
        }
        false
    }

    pub fn registrar_atencion(&mut self, atencion: &RegistroAtencion) {
        self.registro_atencion.push(atencion.clone());
    }

    pub fn buscar_registro_atencion(
        &self,
        nombre_mascota: String,
        nombre_dueño: String,
        telefono: String,
    ) -> Option<RegistroAtencion> {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].comparar(&nombre_mascota, &nombre_dueño, &telefono) {
                return Some(self.registro_atencion[i].clone());
            }
        }
        None
    }

    pub fn modificar_diagnostico(
        &mut self,
        ra_original: &RegistroAtencion,
        ra_modificado: &RegistroAtencion,
    ) -> Option<RegistroAtencion> {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].comparar_completo(ra_original) {
                self.registro_atencion[i] = ra_modificado.clone();
                return Some(self.registro_atencion[i].clone());
            }
        }
        None
    }

    pub fn modificar_fecha_atencion(
        &mut self,
        ra: &RegistroAtencion,
        f: Fecha,
    ) -> Option<RegistroAtencion> {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].comparar_completo(ra) {
                self.registro_atencion[i].proxima_fecha = Some(f);
                return Some(self.registro_atencion[i].clone());
            }
        }
        None
    }

    pub fn eliminar_atencion(&mut self, ra: &RegistroAtencion) -> bool {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].comparar_completo(ra) {
                self.registro_atencion.remove(i);
                return true;
            }
        }
        false
    }
}

impl Mascota {
    pub fn new(nombre: String, edad: u8, tipo: TipoAnimal, cliente: Cliente) -> Self {
        Mascota {
            nombre,
            edad,
            tipo,
            dueño: cliente,
        }
    }

    pub fn comparar(&self, m: &Mascota) -> bool {
        m.nombre == self.nombre && m.dueño.comparar(&self.dueño)
    }
}

impl Cliente {
    pub fn new(direccion: String, nombre: String, telefono: String) -> Self {
        Cliente {
            direccion,
            nombre,
            telefono,
        }
    }

    pub fn comparar(&self, c: &Cliente) -> bool {
        self.direccion == c.direccion && self.nombre == c.nombre
    }
}

impl RegistroAtencion {
    pub fn new(mascota: &Mascota, diagnostico: String, tratamiento: String, fecha: Fecha) -> Self {
        RegistroAtencion {
            mascota: mascota.clone(),
            diagnostico,
            tratamiento,
            fecha,
            proxima_fecha: None,
        }
    }

    pub fn comparar(
        &self,
        nombre_mascota: &String,
        nombre_dueño: &String,
        telefono: &String,
    ) -> bool {
        self.mascota.nombre == *nombre_mascota
            && self.mascota.dueño.nombre == *nombre_dueño
            && self.mascota.dueño.telefono == *telefono
    }

    pub fn comparar_completo(&self, a: &RegistroAtencion) -> bool {
        self.mascota.comparar(&a.mascota) && self.fecha.comparar(&a.fecha)
    }

    pub fn get_info(&self) -> String {
        let dia = self
            .proxima_fecha
            .clone()
            .unwrap_or(Fecha::new(0, 0, 0))
            .dia;
        format!(
            "diagnostico:{} tratamiento:{} prox_dia:{}",
            self.diagnostico, self.tratamiento, dia
        )
    }
}

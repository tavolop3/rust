#![allow(unused_variables, dead_code)]

use crate::tp03::ej03::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

pub struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u64,
    cola_atencion: VecDeque<Mascota>,
    registro_atencion: Vec<RegistroAtencion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mascota {
    nombre: String,
    edad: u8,
    tipo: TipoAnimal,
    dueño: Cliente,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        self.persistir_registros_atencion();
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
                self.persistir_registros_atencion();
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
                self.persistir_registros_atencion();
                return Some(self.registro_atencion[i].clone());
            }
        }
        None
    }

    pub fn eliminar_atencion(&mut self, ra: &RegistroAtencion) -> bool {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].comparar_completo(ra) {
                self.registro_atencion.remove(i);
                self.persistir_registros_atencion();
                return true;
            }
        }
        false
    }

    pub fn persistir_registros_atencion(&self) {
        let mut f = File::create("src/tp05/registros/ej03/registros_atencion.json").unwrap();
        let registros_serializado = serde_json::to_string_pretty(&self.registro_atencion).unwrap();
        f.write_all(registros_serializado.as_bytes()).unwrap();
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
    pub fn new(
        mascota: &Mascota,
        diagnostico: String,
        tratamiento: String,
        fecha: &Fecha,
        proxima_fecha: Option<Fecha>,
    ) -> Self {
        RegistroAtencion {
            mascota: mascota.clone(),
            diagnostico,
            tratamiento,
            fecha: fecha.clone(),
            proxima_fecha,
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

#[cfg(test)]
mod tests {
    use super::*;

    // Estructura para datos de prueba reutilizables
    struct TestData {
        veterinaria: Veterinaria,
        cliente: Cliente,
        mascota_perro: Mascota,
        mascota_gato: Mascota,
        fecha: Fecha,
        registro_atencion: RegistroAtencion,
    }

    // Función setup para inicializar datos de prueba
    fn setup() -> TestData {
        let cliente = Cliente::new(
            "Av. Siempre Viva 123".to_string(),
            "Lucho".to_string(),
            "1234-5678".to_string(),
        );

        let mascota_perro = Mascota::new(
            "Firulais".to_string(),
            5,
            TipoAnimal::Perro,
            cliente.clone(),
        );

        let mascota_gato = Mascota::new("Michi".to_string(), 3, TipoAnimal::Gato, cliente.clone());

        let fecha = Fecha::new(10, 6, 2025);

        let registro_atencion = RegistroAtencion::new(
            &mascota_perro,
            "Fiebre loca".to_string(),
            "Paracetamol canino".to_string(),
            &fecha,
            Some(fecha.clone()),
        );

        let veterinaria =
            Veterinaria::new("Vet Copada".to_string(), "Calle Falsa 456".to_string(), 42);

        TestData {
            veterinaria,
            cliente,
            mascota_perro,
            mascota_gato,
            fecha,
            registro_atencion,
        }
    }

    #[test]
    fn test_new_veterinaria() {
        let data = setup();
        assert_eq!(data.veterinaria.nombre, "Vet Copada");
        assert_eq!(data.veterinaria.direccion, "Calle Falsa 456");
        assert_eq!(data.veterinaria.id, 42);
        assert!(data.veterinaria.cola_atencion.is_empty());
        assert!(data.veterinaria.registro_atencion.is_empty());
    }

    #[test]
    fn test_new_cliente() {
        let data = setup();
        assert_eq!(data.cliente.nombre, "Lucho");
        assert_eq!(data.cliente.direccion, "Av. Siempre Viva 123");
        assert_eq!(data.cliente.telefono, "1234-5678");
    }

    #[test]
    fn test_new_mascota() {
        let data = setup();
        assert_eq!(data.mascota_perro.nombre, "Firulais");
        assert_eq!(data.mascota_perro.edad, 5);
        assert!(matches!(data.mascota_perro.tipo, TipoAnimal::Perro));
        assert_eq!(data.mascota_perro.dueño.nombre, "Lucho");
    }

    #[test]
    fn test_new_registro_atencion() {
        let data = setup();
        assert_eq!(data.registro_atencion.mascota.nombre, "Firulais");
        assert_eq!(data.registro_atencion.diagnostico, "Fiebre loca");
        assert_eq!(data.registro_atencion.tratamiento, "Paracetamol canino");
        assert!(data.registro_atencion.fecha.comparar(&data.fecha));
        assert!(data.registro_atencion.proxima_fecha.is_some());
    }

    #[test]
    fn test_agregar_mascota() {
        let mut data = setup();
        data.veterinaria.agregar_mascota(&data.mascota_perro);
        assert_eq!(data.veterinaria.cola_atencion.len(), 1);
        assert_eq!(data.veterinaria.cola_atencion[0].nombre, "Firulais");
    }

    #[test]
    fn test_agregar_mascota_urgente() {
        let mut data = setup();
        data.veterinaria.agregar_mascota(&data.mascota_gato);
        data.veterinaria
            .agregar_mascota_urgente(&data.mascota_perro);
        assert_eq!(data.veterinaria.cola_atencion.len(), 2);
        assert_eq!(data.veterinaria.cola_atencion[0].nombre, "Firulais");
        assert_eq!(data.veterinaria.cola_atencion[1].nombre, "Michi");
    }

    #[test]
    fn test_atender_mascota() {
        let mut data = setup();
        data.veterinaria.agregar_mascota(&data.mascota_perro);
        data.veterinaria.agregar_mascota(&data.mascota_gato);
        let mascota = data.veterinaria.atender().unwrap();
        assert_eq!(mascota.nombre, "Firulais");
        assert_eq!(data.veterinaria.cola_atencion.len(), 1);
        assert_eq!(data.veterinaria.cola_atencion[0].nombre, "Michi");
    }

    #[test]
    fn test_atender_cola_vacia() {
        let mut data = setup();
        let result = data.veterinaria.atender();
        assert!(result.is_none());
    }

    #[test]
    fn test_eliminar_mascota_existe() {
        let mut data = setup();
        data.veterinaria.agregar_mascota(&data.mascota_perro);
        data.veterinaria.agregar_mascota(&data.mascota_gato);
        let result = data.veterinaria.eliminar_mascota(&data.mascota_perro);
        assert!(result);
        assert_eq!(data.veterinaria.cola_atencion.len(), 1);
        assert_eq!(data.veterinaria.cola_atencion[0].nombre, "Michi");
    }

    #[test]
    fn test_eliminar_mascota_no_existe() {
        let mut data = setup();
        data.veterinaria.agregar_mascota(&data.mascota_gato);
        let mascota_falsa = Mascota::new(
            "Fantasmin".to_string(),
            2,
            TipoAnimal::Caballo,
            data.cliente.clone(),
        );
        let result = data.veterinaria.eliminar_mascota(&mascota_falsa);
        assert!(!result);
        assert_eq!(data.veterinaria.cola_atencion.len(), 1);
    }

    #[test]
    fn test_registrar_atencion() {
        let mut data = setup();
        data.veterinaria.registrar_atencion(&data.registro_atencion);
        assert_eq!(data.veterinaria.registro_atencion.len(), 1);
        assert_eq!(
            data.veterinaria.registro_atencion[0].mascota.nombre,
            "Firulais"
        );
        assert_eq!(
            data.veterinaria.registro_atencion[0].diagnostico,
            "Fiebre loca"
        );
    }

    #[test]
    fn test_buscar_registro_atencion_existe() {
        let mut data = setup();
        data.veterinaria.registrar_atencion(&data.registro_atencion);
        let result = data.veterinaria.buscar_registro_atencion(
            "Firulais".to_string(),
            "Lucho".to_string(),
            "1234-5678".to_string(),
        );
        assert!(result.is_some());
        let registro = result.unwrap();
        assert_eq!(registro.diagnostico, "Fiebre loca");
    }

    #[test]
    fn test_buscar_registro_atencion_no_existe() {
        let mut data = setup();
        data.veterinaria.registrar_atencion(&data.registro_atencion);
        let result = data.veterinaria.buscar_registro_atencion(
            "Michi".to_string(),
            "Lucho".to_string(),
            "1234-5678".to_string(),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_modificar_diagnostico_existe() {
        let mut data = setup();
        data.veterinaria.registrar_atencion(&data.registro_atencion);
        let nuevo_registro = RegistroAtencion::new(
            &data.mascota_perro,
            "Tos perruna".to_string(),
            "Jarabe canino".to_string(),
            &data.fecha,
            Some(data.fecha.clone()),
        );
        let result = data
            .veterinaria
            .modificar_diagnostico(&data.registro_atencion, &nuevo_registro);
        assert!(result.is_some());
        let registro = result.unwrap();
        assert_eq!(registro.diagnostico, "Tos perruna");
        assert_eq!(registro.tratamiento, "Jarabe canino");
        assert_eq!(
            data.veterinaria.registro_atencion[0].diagnostico,
            "Tos perruna"
        );
    }

    #[test]
    fn test_modificar_diagnostico_no_existe() {
        let mut data = setup();
        let registro_falso = RegistroAtencion::new(
            &data.mascota_gato,
            "No existe".to_string(),
            "Nada".to_string(),
            &data.fecha,
            None,
        );
        let nuevo_registro = RegistroAtencion::new(
            &data.mascota_gato,
            "Tos perruna".to_string(),
            "Jarabe canino".to_string(),
            &data.fecha,
            None,
        );
        let result = data
            .veterinaria
            .modificar_diagnostico(&registro_falso, &nuevo_registro);
        assert!(result.is_none());
    }

    #[test]
    fn test_modificar_fecha_atencion_existe() {
        let mut data = setup();
        data.veterinaria.registrar_atencion(&data.registro_atencion);
        let nueva_fecha = Fecha::new(15, 7, 2025);
        let result = data
            .veterinaria
            .modificar_fecha_atencion(&data.registro_atencion, nueva_fecha.clone());
        assert!(result.is_some());
        let registro = result.unwrap();
        assert!(registro.proxima_fecha.is_some());
        assert!(registro.proxima_fecha.unwrap().comparar(&nueva_fecha));
    }

    #[test]
    fn test_modificar_fecha_atencion_no_existe() {
        let mut data = setup();
        let registro_falso = RegistroAtencion::new(
            &data.mascota_gato,
            "No existe".to_string(),
            "Nada".to_string(),
            &data.fecha,
            None,
        );
        let nueva_fecha = Fecha::new(15, 7, 2025);
        let result = data
            .veterinaria
            .modificar_fecha_atencion(&registro_falso, nueva_fecha);
        assert!(result.is_none());
    }

    #[test]
    fn test_eliminar_atencion_existe() {
        let mut data = setup();
        data.veterinaria.registrar_atencion(&data.registro_atencion);
        let result = data.veterinaria.eliminar_atencion(&data.registro_atencion);
        assert!(result);
        assert!(data.veterinaria.registro_atencion.is_empty());
    }

    #[test]
    fn test_eliminar_atencion_no_existe() {
        let mut data = setup();
        let registro_falso = RegistroAtencion::new(
            &data.mascota_gato,
            "No existe".to_string(),
            "Nada".to_string(),
            &data.fecha,
            None,
        );
        let result = data.veterinaria.eliminar_atencion(&registro_falso);
        assert!(!result);
        assert!(data.veterinaria.registro_atencion.is_empty());
    }

    #[test]
    fn test_comparar_mascota() {
        let data = setup();
        let mascota_copia = Mascota::new(
            "Firulais".to_string(),
            10,
            TipoAnimal::Perro,
            data.cliente.clone(),
        );
        assert!(data.mascota_perro.comparar(&mascota_copia));
        assert!(!data.mascota_perro.comparar(&data.mascota_gato));
    }

    #[test]
    fn test_comparar_cliente() {
        let data = setup();
        let cliente_copia = Cliente::new(
            "Av. Siempre Viva 123".to_string(),
            "Lucho".to_string(),
            "9999-9999".to_string(),
        );
        assert!(data.cliente.comparar(&cliente_copia));
        let cliente_diferente = Cliente::new(
            "Otra calle".to_string(),
            "Marto".to_string(),
            "1234-5678".to_string(),
        );
        assert!(!data.cliente.comparar(&cliente_diferente));
    }

    #[test]
    fn test_comparar_registro_atencion() {
        let data = setup();
        assert!(data.registro_atencion.comparar(
            &"Firulais".to_string(),
            &"Lucho".to_string(),
            &"1234-5678".to_string(),
        ));
        assert!(!data.registro_atencion.comparar(
            &"Michi".to_string(),
            &"Lucho".to_string(),
            &"1234-5678".to_string(),
        ));
    }

    #[test]
    fn test_comparar_completo_registro_atencion() {
        let data = setup();
        let registro_copia = RegistroAtencion::new(
            &data.mascota_perro,
            "Otro diagnostico".to_string(),
            "Otro tratamiento".to_string(),
            &data.fecha,
            None,
        );
        assert!(data.registro_atencion.comparar_completo(&registro_copia));
        let registro_diferente = RegistroAtencion::new(
            &data.mascota_gato,
            "Fiebre loca".to_string(),
            "Paracetamol canino".to_string(),
            &data.fecha,
            None,
        );
        assert!(
            !data
                .registro_atencion
                .comparar_completo(&registro_diferente)
        );
    }

    #[test]
    fn test_get_info_registro_atencion() {
        let data = setup();
        let info = data.registro_atencion.get_info();
        assert_eq!(
            info,
            "diagnostico:Fiebre loca tratamiento:Paracetamol canino prox_dia:10"
        );
        let registro_sin_fecha = RegistroAtencion::new(
            &data.mascota_perro,
            "Fiebre loca".to_string(),
            "Paracetamol canino".to_string(),
            &data.fecha,
            None,
        );
        let info = registro_sin_fecha.get_info();
        assert_eq!(
            info,
            "diagnostico:Fiebre loca tratamiento:Paracetamol canino prox_dia:0"
        );
    }
}

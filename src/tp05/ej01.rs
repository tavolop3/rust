#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Write;

pub struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u16,
    autos: Vec<Auto>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Auto {
    marca: String,
    modelo: String,
    año: u32,
    precio_bruto: f64,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Debug, PartialEq)]
pub struct CapacidadError(u16);

impl fmt::Display for CapacidadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Agregar este auto hace que se supere la capacidad maxima {}",
            self.0
        )
    }
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad: u16) -> Self {
        let autos: Vec<Auto> = vec![];
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos,
        }
    }

    // 1a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error propio con un mensaje de contexto.
    pub fn agregar_auto(&mut self, auto: &Auto) -> Result<(), CapacidadError> {
        if self.capacidad == self.autos.len() as u16 {
            return Err(CapacidadError(self.capacidad));
        }

        self.autos.push(auto.clone());
        self.persistir_autos();

        Ok(())
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) {
        self.autos.retain(|a| !a.comparar(auto));
        self.persistir_autos();
    }

    fn persistir_autos(&self) {
        let mut f = File::create("src/tp05/registros/ej01/autos.json").unwrap();
        let auto_serializado = serde_json::to_string_pretty(&self.autos).unwrap();
        f.write_all(auto_serializado.as_bytes()).unwrap();
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<Auto> {
        let mut resul: Option<Auto>;

        for a in &self.autos {
            if a.comparar(auto) {
                return Some(a.clone());
            }
        }
        None
    }
}

impl Auto {
    pub fn new(marca: String, modelo: String, año: u32, precio_bruto: f64, color: Color) -> Self {
        Auto {
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }

    fn comparar(&self, a2: &Auto) -> bool {
        self.marca == a2.marca && self.modelo == a2.modelo
    }

    pub fn get_info(&self) -> String {
        format!("Marca: {}, Modelo: {}", self.marca, self.modelo)
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut tasa = match self.color {
            Color::Rojo | Color::Amarillo | Color::Azul => 0.25,
            _ => -0.10,
        };

        if self.marca == "BMW" {
            tasa += 0.15;
        }

        if self.año < 2000 {
            tasa -= 0.05;
        }

        self.precio_bruto * (1.0 + tasa)
    }
}

// 1- En base al ejercicio 7 del tp#3 implemente lo siguiente:
// a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error propio con un mensaje de contexto.
//
// b- Haga todos los tests correspondientes para probar en profundidad los métodos
// que agregan un auto y eliminan un auto de la concesionaria , obteniendo el mayor
// porcentaje de coverage sobre el código que realiza las operaciones.
//
// c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se
// almacenen en un archivo en formato JSON. Agregue y modifique lo que considere
// necesario para que:
// - Al agregar un nuevo auto se abre el archivo de autos guardados y lo agregue a
// dicho archivo.
// - Eliminar un auto: al eliminar un auto se debe eliminar este del archivo.
// No debe modificar los tests hechos en el punto b. Si puede agregar más en caso de que
// haga nueva funcionalidad..

#[cfg(test)]
mod tests {
    use super::{Auto, Color, ConcesionarioAuto};

    fn crear_auto(marca: &str, modelo: &str, año: u32, precio: f64, color: Color) -> Auto {
        Auto::new(marca.to_string(), modelo.to_string(), año, precio, color)
    }

    #[test]
    fn test_agregar_auto_concesionaria_vacia() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 2);
        let auto = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);

        let result = concesionario.agregar_auto(&auto);
        assert!(result.is_ok(), "Debería agregar el auto exitosamente");
        assert_eq!(
            concesionario.autos.len(),
            1,
            "Debería haber un auto en la lista"
        );
        assert_eq!(
            concesionario.buscar_auto(&auto).map(|a| a.get_info()),
            Some(auto.get_info()),
            "El auto agregado debería estar en la lista"
        );
    }

    #[test]
    fn test_agregar_auto_concesionaria_llena() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 1);
        let auto1 = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);
        let auto2 = crear_auto("Honda", "Civic", 2021, 22000.0, Color::Rojo);

        assert!(
            concesionario.agregar_auto(&auto1).is_ok(),
            "Debería agregar el primer auto"
        );
        let result = concesionario.agregar_auto(&auto2);
        assert!(
            result.is_err(),
            "Debería fallar al agregar un auto cuando está llena"
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Agregar este auto hace que se supere la capacidad maxima 1",
            "El mensaje de error debería ser correcto"
        );
        assert_eq!(concesionario.autos.len(), 1, "La lista no debería crecer");
        assert!(
            concesionario.buscar_auto(&auto2).is_none(),
            "El segundo auto no debería estar en la lista"
        );
    }

    #[test]
    fn test_agregar_auto_hasta_capacidad() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 3);
        let auto1 = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);
        let auto2 = crear_auto("Honda", "Civic", 2021, 22000.0, Color::Rojo);
        let auto3 = crear_auto("Ford", "Focus", 2019, 18000.0, Color::Blanco);
        let auto4 = crear_auto("BMW", "X5", 2022, 50000.0, Color::Negro);

        assert!(
            concesionario.agregar_auto(&auto1).is_ok(),
            "Debería agregar el primer auto"
        );
        assert!(
            concesionario.agregar_auto(&auto2).is_ok(),
            "Debería agregar el segundo auto"
        );
        assert!(
            concesionario.agregar_auto(&auto3).is_ok(),
            "Debería agregar el tercer auto"
        );
        assert!(
            concesionario.agregar_auto(&auto4).is_err(),
            "Debería fallar al agregar el cuarto auto"
        );
        assert_eq!(
            concesionario.autos.len(),
            3,
            "Debería haber tres autos en la lista"
        );
    }

    #[test]
    fn test_eliminar_auto_existente() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 2);
        let auto1 = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);
        let auto2 = crear_auto("Honda", "Civic", 2021, 22000.0, Color::Rojo);

        concesionario.agregar_auto(&auto1).unwrap();
        concesionario.agregar_auto(&auto2).unwrap();
        concesionario.eliminar_auto(&auto1);

        assert_eq!(
            concesionario.autos.len(),
            1,
            "Debería quedar un auto en la lista"
        );
        assert!(
            concesionario.buscar_auto(&auto1).is_none(),
            "El auto eliminado no debería estar en la lista"
        );
        assert!(
            concesionario.buscar_auto(&auto2).is_some(),
            "El otro auto debería seguir en la lista"
        );
    }

    #[test]
    fn test_eliminar_auto_no_existente() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 2);
        let auto1 = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);
        let auto2 = crear_auto("Honda", "Civic", 2021, 22000.0, Color::Rojo);

        concesionario.agregar_auto(&auto1).unwrap();
        concesionario.eliminar_auto(&auto2);

        assert_eq!(concesionario.autos.len(), 1, "La lista no debería cambiar");
        assert!(
            concesionario.buscar_auto(&auto1).is_some(),
            "El auto original debería seguir en la lista"
        );
    }

    #[test]
    fn test_eliminar_auto_concesionaria_vacia() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 2);
        let auto = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);

        concesionario.eliminar_auto(&auto);

        assert_eq!(
            concesionario.autos.len(),
            0,
            "La lista debería seguir vacía"
        );
    }

    #[test]
    fn test_comparar_auto_ignora_atributos_no_relevantes() {
        let mut concesionario =
            ConcesionarioAuto::new("Test".to_string(), "Calle 1".to_string(), 2);
        let auto1 = crear_auto("Toyota", "Corolla", 2020, 20000.0, Color::Azul);
        let auto2 = crear_auto("Toyota", "Corolla", 2019, 18000.0, Color::Rojo);

        concesionario.agregar_auto(&auto1).unwrap();
        concesionario.eliminar_auto(&auto2);

        assert_eq!(
            concesionario.autos.len(),
            0,
            "Debería eliminar el auto por marca y modelo"
        );
        assert!(
            concesionario.buscar_auto(&auto1).is_none(),
            "El auto no debería estar en la lista"
        );
    }

    #[test]
    fn test_ej01_concesionario() {
        let mut c = ConcesionarioAuto::new("Concesionario".to_string(), "Calle 1".to_string(), 1);
        let a1 = crear_auto("Audi", "A3", 1999, 100.0, Color::Negro);
        let a2 = crear_auto("BMW", "A4", 2015, 100.0, Color::Rojo);

        assert!(c.agregar_auto(&a1).is_ok(), "Debería agregar a1");
        assert!(
            c.agregar_auto(&a2).is_err(),
            "No debería agregar a2 por capacidad"
        );
        assert_eq!(
            c.buscar_auto(&a1).map(|a| a.get_info()),
            Some(a1.get_info()),
            "Debería encontrar a1"
        );
        assert!(c.buscar_auto(&a2).is_none(), "No debería encontrar a2");
        c.eliminar_auto(&a1);
        assert!(
            c.buscar_auto(&a1).is_none(),
            "a1 debería haber sido eliminado"
        );
        assert_eq!(a1.calcular_precio(), 85.0, "Precio de a1 debería ser 85.0");
        assert_eq!(
            a2.calcular_precio(),
            140.0,
            "Precio de a2 debería ser 140.0"
        );
    }
}

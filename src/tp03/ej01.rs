#[derive(Debug)]
pub struct Persona {
    nombre: String,
    direccion: Option<String>,
    edad: u8,
}

impl Persona {
    pub fn new(nombre: String, direccion: Option<String>, edad: u8) -> Self {
        Persona {
            nombre,
            direccion,
            edad,
        }
    }

    pub fn to_string(&self) -> String {
        let dir_str = match &self.direccion {
            Some(dir) => dir.clone(),
            None => String::from("Sin dirección"),
        };
        format!(
            "Nombre:{0}, Direccion: {dir_str}, Edad:{1}",
            self.nombre, self.edad
        )
    }

    pub fn obtener_edad(&self) -> u8 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_dir: String) {
        self.direccion = Some(nueva_dir);
    }
}

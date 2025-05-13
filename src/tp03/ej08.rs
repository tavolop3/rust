#![allow(dead_code, unused_variables)]

#[derive(Clone, Debug)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

#[derive(Clone, Debug)]
pub enum Genero {
    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS,
}

#[derive(Debug)]
pub struct Playlist {
    canciones: Vec<Cancion>,
    nombre: String,
}

impl Cancion {
    pub fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }

    pub fn comparar(&self, c: &Cancion) -> bool {
        if self.titulo == c.titulo && self.artista == c.artista {
            return true;
        }
        false
    }

    pub fn info(&self) -> String {
        format!("Titulo:{}, Artista: {}", self.titulo, self.artista)
    }
}

impl Playlist {
    pub fn new(nombre: String) -> Playlist {
        Playlist {
            canciones: vec![],
            nombre,
        }
    }

    pub fn agregar_cancion(&mut self, c: &Cancion) {
        self.canciones.push(c.clone());
    }

    pub fn eliminar_cancion(&mut self, c: &Cancion) {
        for i in 0..self.canciones.len() {
            if self.canciones[i].comparar(c) {
                self.canciones.remove(i);
                return;
            }
        }
    }

    pub fn mover_cancion(&mut self, c: &Cancion, pos: usize) {
        if pos >= self.canciones.len() {
            return;
        }

        for i in 0..self.canciones.len() {
            if self.canciones[i].comparar(c) {
                self.canciones.swap(i, pos);
                return;
            }
        }
    }

    pub fn buscar_cancion_por_nombre(&self, nombre: String) -> Option<Cancion> {
        for c in &self.canciones {
            println!("{}", c.titulo);
            if c.titulo == nombre {
                return Some(c.clone());
            }
        }
        None
    }

    pub fn get_posicion_cancion(&self, c: &Cancion) -> Option<usize> {
        for i in 0..self.canciones.len() {
            if self.canciones[i].comparar(c) {
                return Some(i);
            }
        }
        None
    }

    pub fn get_canciones_genero(&self, genero: &Genero) -> Vec<&Cancion> {
        let mut cs: Vec<&Cancion> = Vec::new();
        for c in &self.canciones {
            if c.genero.igual(genero) {
                cs.push(c);
            }
        }
        cs
    }

    pub fn get_canciones_artista(&self, artista: String) -> Vec<&Cancion> {
        let mut cs: Vec<&Cancion> = Vec::new();
        for c in &self.canciones {
            if c.artista == artista {
                cs.push(c);
            }
        }
        cs
    }

    pub fn cambiar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    pub fn del_all_canciones(&mut self) {
        self.canciones.clear();
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn get_len_canciones(&self) -> usize {
        self.canciones.len()
    }
}

impl Genero {
    pub fn igual(&self, genero: &Genero) -> bool {
        self.a_str() == genero.a_str()
    }

    pub fn a_str(&self) -> String {
        match self {
            Genero::ROCK => String::from("ROCK"),
            Genero::POP => String::from("POP"),
            Genero::RAP => String::from("RAP"),
            Genero::JAZZ => String::from("JAZZ"),
            Genero::OTROS => String::from("OTROS"),
        }
    }
}

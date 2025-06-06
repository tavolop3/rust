#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
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

    pub fn persistir_canciones(&self) {
        let mut f = File::create("src/tp05/canciones.json").unwrap();
        let canciones_serializado = serde_json::to_string_pretty(&self.canciones).unwrap();
        f.write_all(canciones_serializado.as_bytes()).unwrap();
    }

    pub fn agregar_cancion(&mut self, c: &Cancion) {
        self.canciones.push(c.clone());
        self.persistir_canciones();
    }

    pub fn eliminar_cancion(&mut self, c: &Cancion) {
        for i in 0..self.canciones.len() {
            if self.canciones[i].comparar(c) {
                self.canciones.remove(i);
                return;
            }
        }
        self.persistir_canciones();
    }

    pub fn mover_cancion(&mut self, c: &Cancion, pos: usize) {
        if pos >= self.canciones.len() {
            return;
        }

        for i in 0..self.canciones.len() {
            if self.canciones[i].comparar(c) {
                let c = self.canciones.remove(i);
                self.canciones.insert(pos, c);
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
        self.persistir_canciones();
    }

    pub fn del_all_canciones(&mut self) {
        self.canciones.clear();
        self.persistir_canciones();
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
            Genero::Rock => String::from("ROCK"),
            Genero::Pop => String::from("POP"),
            Genero::Rap => String::from("RAP"),
            Genero::Jazz => String::from("JAZZ"),
            Genero::Otros => String::from("OTROS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cancion, Genero, Playlist};

    fn crear_cancion(titulo: &str, artista: &str, genero: Genero) -> Cancion {
        Cancion::new(titulo.to_string(), artista.to_string(), genero)
    }

    #[test]
    fn test_cancion_new_and_info() {
        let cancion = crear_cancion("Bohemian Rhapsody", "Queen", Genero::Rock);
        assert_eq!(cancion.titulo, "Bohemian Rhapsody");
        assert_eq!(cancion.artista, "Queen");
        assert!(matches!(cancion.genero, Genero::Rock));
        assert_eq!(cancion.info(), "Titulo:Bohemian Rhapsody, Artista: Queen");
    }

    #[test]
    fn test_cancion_comparar() {
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song1", "Artist1", Genero::Pop);
        let cancion3 = crear_cancion("Song2", "Artist1", Genero::Rock);
        let cancion4 = crear_cancion("Song1", "Artist2", Genero::Rock);

        assert!(
            cancion1.comparar(&cancion2),
            "Songs with same title and artist should be equal"
        );
        assert!(
            !cancion1.comparar(&cancion3),
            "Different titles should not be equal"
        );
        assert!(
            !cancion1.comparar(&cancion4),
            "Different artists should not be equal"
        );
    }

    #[test]
    fn test_genero_igual_y_a_str() {
        assert!(
            Genero::Rock.igual(&Genero::Rock),
            "Same genre should be equal"
        );
        assert!(
            !Genero::Rock.igual(&Genero::Pop),
            "Different genres should not be equal"
        );
        assert_eq!(Genero::Rock.a_str(), "ROCK");
        assert_eq!(Genero::Pop.a_str(), "POP");
        assert_eq!(Genero::Rap.a_str(), "RAP");
        assert_eq!(Genero::Jazz.a_str(), "JAZZ");
        assert_eq!(Genero::Otros.a_str(), "OTROS");
    }

    #[test]
    fn test_playlist_new() {
        let playlist = Playlist::new("My Playlist".to_string());
        assert_eq!(playlist.get_nombre(), "My Playlist");
        assert_eq!(playlist.get_len_canciones(), 0);
        assert!(playlist.canciones.is_empty());
    }

    #[test]
    fn test_agregar_cancion() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion = crear_cancion("Song1", "Artist1", Genero::Rock);

        playlist.agregar_cancion(&cancion);
        assert_eq!(playlist.get_len_canciones(), 1);
        assert_eq!(playlist.canciones[0].info(), cancion.info());
    }

    #[test]
    fn test_eliminar_cancion_existente() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        assert_eq!(playlist.get_len_canciones(), 2);

        playlist.eliminar_cancion(&cancion1);
        assert_eq!(playlist.get_len_canciones(), 1);
        assert_eq!(playlist.canciones[0].info(), cancion2.info());
    }

    #[test]
    fn test_eliminar_cancion_no_existente() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);

        playlist.agregar_cancion(&cancion1);
        playlist.eliminar_cancion(&cancion2);
        assert_eq!(playlist.get_len_canciones(), 1);
        assert_eq!(playlist.canciones[0].info(), cancion1.info());
    }

    #[test]
    fn test_eliminar_cancion_lista_vacia() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion = crear_cancion("Song1", "Artist1", Genero::Rock);

        playlist.eliminar_cancion(&cancion);
        assert_eq!(playlist.get_len_canciones(), 0);
    }

    #[test]
    fn test_mover_cancion_valida() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);
        let cancion3 = crear_cancion("Song3", "Artist3", Genero::Jazz);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        playlist.mover_cancion(&cancion1, 2);
        assert_eq!(playlist.canciones[2].info(), cancion1.info());
        assert_eq!(playlist.canciones[0].info(), cancion2.info());
        assert_eq!(playlist.canciones[1].info(), cancion3.info());
    }

    #[test]
    fn test_mover_cancion_posicion_invalida() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion = crear_cancion("Song1", "Artist1", Genero::Rock);

        playlist.agregar_cancion(&cancion);
        let original = playlist.canciones.clone();
        playlist.mover_cancion(&cancion, 5); // Posición inválida
        assert_eq!(playlist.canciones, original, "Playlist should not change");
    }

    #[test]
    fn test_mover_cancion_no_existente() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);

        playlist.agregar_cancion(&cancion1);
        let original = playlist.canciones.clone();
        playlist.mover_cancion(&cancion2, 0);
        assert_eq!(playlist.canciones, original, "Playlist should not change");
    }

    #[test]
    fn test_buscar_cancion_por_nombre_existente() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion = crear_cancion("Song1", "Artist1", Genero::Rock);

        playlist.agregar_cancion(&cancion);
        let result = playlist.buscar_cancion_por_nombre("Song1".to_string());
        assert!(result.is_some());
        assert_eq!(result.unwrap().info(), cancion.info());
    }

    #[test]
    fn test_buscar_cancion_por_nombre_no_existente() {
        let playlist = Playlist::new("Test Playlist".to_string());
        let result = playlist.buscar_cancion_por_nombre("Nonexistent".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_get_posicion_cancion() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);

        assert_eq!(playlist.get_posicion_cancion(&cancion1), Some(0));
        assert_eq!(playlist.get_posicion_cancion(&cancion2), Some(1));
        let cancion3 = crear_cancion("Song3", "Artist3", Genero::Jazz);
        assert_eq!(playlist.get_posicion_cancion(&cancion3), None);
    }

    #[test]
    fn test_get_canciones_genero() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);
        let cancion3 = crear_cancion("Song3", "Artist1", Genero::Rock);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        let rock_songs = playlist.get_canciones_genero(&Genero::Rock);
        assert_eq!(rock_songs.len(), 2);
        assert!(rock_songs.iter().any(|c| c.info() == cancion1.info()));
        assert!(rock_songs.iter().any(|c| c.info() == cancion3.info()));

        let pop_songs = playlist.get_canciones_genero(&Genero::Pop);
        assert_eq!(pop_songs.len(), 1);
        assert!(pop_songs.iter().any(|c| c.info() == cancion2.info()));

        let jazz_songs = playlist.get_canciones_genero(&Genero::Jazz);
        assert_eq!(jazz_songs.len(), 0);
    }

    #[test]
    fn test_get_canciones_artista() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion1 = crear_cancion("Song1", "Artist1", Genero::Rock);
        let cancion2 = crear_cancion("Song2", "Artist2", Genero::Pop);
        let cancion3 = crear_cancion("Song3", "Artist1", Genero::Jazz);

        playlist.agregar_cancion(&cancion1);
        playlist.agregar_cancion(&cancion2);
        playlist.agregar_cancion(&cancion3);

        let artist1_songs = playlist.get_canciones_artista("Artist1".to_string());
        assert_eq!(artist1_songs.len(), 2);
        assert!(artist1_songs.iter().any(|c| c.info() == cancion1.info()));
        assert!(artist1_songs.iter().any(|c| c.info() == cancion3.info()));

        let artist2_songs = playlist.get_canciones_artista("Artist2".to_string());
        assert_eq!(artist2_songs.len(), 1);
        assert!(artist2_songs.iter().any(|c| c.info() == cancion2.info()));

        let unknown_songs = playlist.get_canciones_artista("Unknown".to_string());
        assert_eq!(unknown_songs.len(), 0);
    }

    #[test]
    fn test_cambiar_titulo() {
        let mut playlist = Playlist::new("Old Playlist".to_string());
        playlist.cambiar_titulo("New Playlist".to_string());
        assert_eq!(playlist.get_nombre(), "New Playlist");
    }

    #[test]
    fn test_del_all_canciones() {
        let mut playlist = Playlist::new("Test Playlist".to_string());
        let cancion = crear_cancion("Song1", "Artist1", Genero::Rock);
        playlist.agregar_cancion(&cancion);
        assert_eq!(playlist.get_len_canciones(), 1);

        playlist.del_all_canciones();
        assert_eq!(playlist.get_len_canciones(), 0);
        assert!(playlist.canciones.is_empty());
    }

    #[test]
    fn test_get_len_canciones_empty() {
        let playlist = Playlist::new("Test Playlist".to_string());
        assert_eq!(playlist.get_len_canciones(), 0);
    }
}

/* Enunciado:
b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser
guardadas en un archivo en formato JSON, por lo tanto las operaciones que agreguen,
quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.
 */

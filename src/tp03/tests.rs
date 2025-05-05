#[cfg(test)]
mod tests {
    use crate::tp03::ej01::Persona;

    #[test]
    fn test_ej01_persona() {
        let mut p = Persona::new("Tao".to_string(), Some("1 e/2 y 3".to_string()), 33);
        assert_eq!(p.obtener_edad(), 33);

        p.actualizar_direccion("marte".to_string());
        assert_eq!("Nombre:Tao, Direccion: marte, Edad:33".to_string(), p.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::tp03::ej01::Persona;
    use crate::tp03::ej02::Rectangulo;
    use crate::tp03::ej03::Fecha;

    #[test]
    fn test_ej01_persona() {
        let mut p = Persona::new("Tao".to_string(), Some("1 e/2 y 3".to_string()), 33);
        assert_eq!(p.obtener_edad(), 33);

        p.actualizar_direccion("marte".to_string());
        assert_eq!("Nombre:Tao, Direccion: marte, Edad:33".to_string(), p.to_string());
    }

    #[test]
    fn test_ej02_rectangulo() {
        let r = Rectangulo::new(30.0,30.0);
        assert_eq!(r.calcular_area(), 900.0);
        assert_eq!(r.calcular_perimetro(), 120.0);
        assert_eq!(r.es_cuadrado(), true);
    }

    #[test]
    fn test_ej03_fechas() {
        let mut f = Fecha::new(15,10,2001);
        assert_eq!(f.es_fecha_valida(), true);
        assert_eq!(f.es_bisiesto(), false);
        f.sumar_dias(10);
        assert_eq!(25, f.dia);
        f.restar_dias(10);
        assert_eq!(15, f.dia);
        f.sumar_dias(183);
        f.sumar_dias(183);
        assert_eq!(2002, f.año);
        let f_inv = Fecha::new(32, 7, 2023);
        assert_eq!(f_inv.es_fecha_valida(), false);
        let f_may = Fecha::new(1, 5, 2025);
        assert_eq!(f_may.es_mayor(f), true);
        let f_bisiesto = Fecha::new(1,1,2020);
        assert_eq!(f_bisiesto.es_bisiesto(), true);
    }
}

#[cfg(test)]
mod tests_tp03 {
    use crate::tp03::ej03::Fecha;

    #[test]
    fn test_ej01_persona() {
        use crate::tp03::ej01::Persona;
        let mut p = Persona::new("Tao".to_string(), Some("1 e/2 y 3".to_string()), 33);
        assert_eq!(p.obtener_edad(), 33);

        p.actualizar_direccion("marte".to_string());
        assert_eq!(
            "Nombre:Tao, Direccion: marte, Edad:33".to_string(),
            p.to_string()
        );
    }

    #[test]
    fn test_ej02_rectangulo() {
        use crate::tp03::ej02::Rectangulo;
        let r = Rectangulo::new(30.0, 30.0);
        assert_eq!(r.calcular_area(), 900.0);
        assert_eq!(r.calcular_perimetro(), 120.0);
        assert!(r.es_cuadrado());
    }

    #[test]
    fn test_ej03_fechas() {
        let mut f = Fecha::new(15, 10, 2001);
        assert!(f.es_fecha_valida());
        assert!(!f.es_bisiesto());
        f.sumar_dias(10);
        assert_eq!(25, f.dia);
        f.restar_dias(10);
        assert_eq!(15, f.dia);
        f.sumar_dias(183);
        f.sumar_dias(183);
        assert_eq!(2002, f.año);
        let f_inv = Fecha::new(32, 7, 2023);
        assert!(!f_inv.es_fecha_valida());
        let f_may = Fecha::new(1, 5, 2025);
        assert!(f_may.es_mayor(&f));
        let f_bisiesto = Fecha::new(1, 1, 2020);
        assert!(f_bisiesto.es_bisiesto());
    }

    #[test]
    fn test_ej04_triangulo() {
        use crate::tp03::ej04::TipoTriangulo;
        use crate::tp03::ej04::Triangulo;

        let t_eq = Triangulo::new(1.0, 1.0, 1.0);
        let t_is = Triangulo::new(5.0, 5.0, 6.0);
        let t_es = Triangulo::new(3.0, 4.0, 5.0);
        assert!(matches!(t_eq.determinar_tipo(), TipoTriangulo::EQUILATERO));
        assert!(matches!(t_is.determinar_tipo(), TipoTriangulo::ISOCELES));
        assert!(matches!(t_es.determinar_tipo(), TipoTriangulo::ESCALENO));
        assert_eq!(t_eq.calcular_area(), 0.4330127);
        assert_eq!(t_is.calcular_area(), 12.0);
        assert_eq!(t_es.calcular_area(), 6.0);
        assert_eq!(t_eq.calcular_perimetro(), 3.0);
        assert_eq!(t_is.calcular_perimetro(), 16.0);
        assert_eq!(t_es.calcular_perimetro(), 12.0);
    }

    #[test]
    fn test_ej05_producto() {
        use crate::tp03::ej05::Producto;

        let p = Producto::new("Mate", 7000.0, 0);
        let tot_imp = 7000.0 * 0.21;
        let tot_desc = 7000.0 - 7000.0 * 0.10;
        let tot = tot_desc + tot_imp;
        assert_eq!(p.calcular_impuestos(21.0), tot_imp);
        assert_eq!(p.aplicar_descuento(10.0), tot_desc);
        assert_eq!(p.calcular_precio_total(Some(21.0), Some(10.0)), tot);
        assert_eq!(
            p.calcular_precio_total(Some(21.0), None),
            (tot + 7000.0 * 0.10)
        );
        assert_eq!(p.calcular_precio_total(None, None), 7000.0);
    }

    #[test]
    fn test_ej06_examenes() {
        use crate::tp03::ej06::Estudiante;
        use crate::tp03::ej06::Examen;

        let ex1 = Examen::new("Rust", 8.5);
        let ex2 = Examen::new("OO2", 7.0);
        let ex3 = Examen::new("SO", 9.0);
        let est = Estudiante::new("Alan Turing", 1, vec![ex1, ex2, ex3]);
        assert_eq!(est.obtener_promedio(), (8.5 + 7.0 + 9.0) / 3.0);
        assert_eq!(est.obtener_calificacion_mas_alta(), 9.0);
        assert_eq!(est.obtener_calificacion_mas_baja(), 7.0);
        let est_vacio = Estudiante::new("Ana Gómez", 67890, vec![]);

        assert_eq!(est_vacio.obtener_promedio(), 0.0);
        assert_eq!(est_vacio.obtener_calificacion_mas_alta(), 0.0);
        assert_eq!(est_vacio.obtener_calificacion_mas_baja(), 0.0);
    }

    #[test]
    fn test_ej07_concesionario() {
        use crate::tp03::ej07::Auto;
        use crate::tp03::ej07::Color;
        use crate::tp03::ej07::ConcesionarioAuto;

        let a1 = Auto::new(
            "Audi".to_string(),
            "A3".to_string(),
            1999,
            100.0,
            Color::NEGRO,
        );
        let mut c = ConcesionarioAuto::new("Concesionario".to_string(), "Calle 1".to_string(), 1);
        let a2 = Auto::new(
            "BMW".to_string(),
            "A4".to_string(),
            2015,
            100.0,
            Color::ROJO,
        );
        assert!(c.agregar_auto(&a1));
        assert!(!c.agregar_auto(&a2));
        assert_eq!(c.buscar_auto(&a1).unwrap().get_info(), a1.get_info());
        assert!(c.buscar_auto(&a2).is_none());
        c.eliminar_auto(&a1);
        assert!(c.buscar_auto(&a1).is_none());
        assert_eq!(a1.calcular_precio(), 85.0);
        assert_eq!(a2.calcular_precio(), 140.0);
    }

    #[test]
    fn test_ej08_playlist() {
        use crate::tp03::ej08::Cancion;
        use crate::tp03::ej08::Genero;
        use crate::tp03::ej08::Playlist;

        let c1 = Cancion::new(
            String::from("My Propeller"),
            String::from("Arctic Monkeys"),
            Genero::ROCK,
        );
        let mut p = Playlist::new(String::from("Monos"));
        p.agregar_cancion(&c1);
        let c = p.buscar_cancion_por_nombre(String::from("My Propeller"));
        assert_eq!(c.unwrap().info(), c1.info());
        p.eliminar_cancion(&c1);
        let c = p.buscar_cancion_por_nombre(String::from("My Propeller"));
        assert!(c.is_none());

        let c0 = Cancion::new(
            String::from("Dance little liar"),
            String::from("Arctic Monkeys"),
            Genero::ROCK,
        );
        p.agregar_cancion(&c0);
        let c1 = Cancion::new(
            String::from("Cornerstone"),
            String::from("Arctic Monkeys"),
            Genero::ROCK,
        );
        p.agregar_cancion(&c1);

        assert_eq!(p.get_posicion_cancion(&c1).unwrap(), 1);
        p.mover_cancion(&c1, 0);
        assert_eq!(p.get_posicion_cancion(&c1).unwrap(), 0);

        assert!(p.get_canciones_genero(&Genero::ROCK)[0].comparar(&c1));
        assert_eq!(
            p.get_canciones_artista(String::from("Arctic Monkeys"))
                .len(),
            2
        );
        assert_eq!(*p.get_nombre(), String::from("Monos"));
        p.cambiar_titulo(String::from("Articos"));
        assert_eq!(*p.get_nombre(), String::from("Articos"));

        assert_eq!(p.get_len_canciones(), 2);
        p.del_all_canciones();
        assert_eq!(p.get_len_canciones(), 0);
    }

    #[test]
    fn test_ej09_veterinaria() {
        use crate::tp03::ej09::Cliente;
        use crate::tp03::ej09::Mascota;
        use crate::tp03::ej09::RegistroAtencion;
        use crate::tp03::ej09::TipoAnimal;
        use crate::tp03::ej09::Veterinaria;

        let mut v = Veterinaria::new(String::from("WideArrow"), String::from("Calle 1"), 1);
        let c1 = Cliente::new("calle 1".to_string(), "tao".to_string(), "123".to_string());
        let m1 = Mascota::new("bobi".to_string(), 1, TipoAnimal::Perro, c1);
        let ra1 = RegistroAtencion::new(
            &m1,
            "ta joya".to_string(),
            "nada".to_string(),
            Fecha::new(8, 5, 2024),
        );
        let c2 = Cliente::new("calle 2".to_string(), "topa".to_string(), "234".to_string());
        let m2 = Mascota::new("toby".to_string(), 2, TipoAnimal::Perro, c2);
        let ra2 = RegistroAtencion::new(
            &m2,
            "ta joya".to_string(),
            "nada".to_string(),
            Fecha::new(9, 6, 2023),
        );
        let ra2_mod = RegistroAtencion::new(
            &m2,
            "no está joya".to_string(),
            "si tiene algo che".to_string(),
            Fecha::new(9, 6, 2023),
        );

        v.agregar_mascota(&m1);
        assert!(v.atender().is_some());
        v.eliminar_mascota(&m1);
        assert!(v.atender().is_none());
        v.agregar_mascota(&m1);
        v.agregar_mascota_urgente(&m2);
        assert!(v.atender().unwrap().comparar(&m2));

        v.agregar_mascota(&m2);
        v.registrar_atencion(&ra1);
        v.registrar_atencion(&ra2);
        assert!(
            v.buscar_registro_atencion("bobi".to_string(), "tao".to_string(), "123".to_string())
                .is_some()
        );
        assert!(
            v.buscar_registro_atencion(
                "truki".to_string(),
                "nao existe".to_string(),
                "123".to_string()
            )
            .is_none()
        );
        assert!(
            v.buscar_registro_atencion("toby".to_string(), "topa".to_string(), "234".to_string())
                .is_some()
        );

        assert_eq!(
            v.modificar_diagnostico(&ra2, &ra2_mod).unwrap().get_info(),
            "diagnostico:no está joya tratamiento:si tiene algo che prox_dia:0"
        );
        assert_eq!(
            v.modificar_fecha_atencion(&ra2, Fecha::new(15, 10, 2023))
                .unwrap()
                .get_info(),
            "diagnostico:no está joya tratamiento:si tiene algo che prox_dia:15"
        );
        assert_eq!(
            v.modificar_fecha_atencion(&ra2, Fecha::new(15, 10, 2023))
                .unwrap()
                .proxima_fecha
                .unwrap()
                .dia,
            15
        );

        assert!(v.eliminar_atencion(&ra2));
        assert!(!v.eliminar_atencion(&ra2));
    }

    #[test]
    fn test_ej10_biblioteca() {
        use crate::tp03::ej10::*;

        let mut biblioteca = Biblioteca::new(
            "Biblioteca informatica".to_string(),
            "Calle 123".to_string(),
        );
        let libro1 = Libro::new(
            "12345".to_string(),
            "Un libro".to_string(),
            "Un autor1".to_string(),
            400,
            Genero::Novela,
        );
        let libro2 = Libro::new(
            "67890".to_string(),
            "La biblia de C".to_string(),
            "Brian Cranston".to_string(),
            120,
            Genero::Tecnico,
        );

        let cliente1 = Cliente::new(
            "Juan Perez".to_string(),
            "123456789".to_string(),
            "juan@email.com".to_string(),
        );
        let cliente2 = Cliente::new(
            "Pepito Lopez".to_string(),
            "987654321".to_string(),
            "pepito@email.com".to_string(),
        );

        biblioteca.agregar_libro(&libro1, 5);
        biblioteca.agregar_libro(&libro2, 4);

        assert_eq!(biblioteca.cant_disponibles(&libro1), 5);
        assert_eq!(biblioteca.cant_disponibles(&libro2), 4);

        biblioteca.decrementar_disponibilidad(&libro1);
        assert_eq!(biblioteca.cant_disponibles(&libro1), 4);

        biblioteca.incrementar_disponibilidad(&libro1);
        assert_eq!(biblioteca.cant_disponibles(&libro1), 5);

        // Probar préstamo
        let mut fecha_vencimiento = Fecha::fecha_actual();
        fecha_vencimiento.sumar_dias(7);
        assert!(biblioteca.prestar(&cliente1, &libro1, &fecha_vencimiento));
        assert_eq!(biblioteca.cant_disponibles(&libro1), 4);
        assert_eq!(biblioteca.cant_prestamos_cli(&cliente1), 1);

        // Probar límite de 5 préstamos
        for _ in 0..4 {
            assert!(biblioteca.prestar(&cliente1, &libro2, &fecha_vencimiento));
        }
        assert_eq!(biblioteca.cant_prestamos_cli(&cliente1), 5);
        assert!(!biblioteca.prestar(&cliente1, &libro1, &fecha_vencimiento)); // falla por límite

        // Probar préstamo sin copias
        for _ in 0..2 {
            biblioteca.decrementar_disponibilidad(&libro2);
        }
        assert!(!biblioteca.prestar(&cliente2, &libro2, &fecha_vencimiento)); // falla por no haber copias

        // Probar buscar préstamo
        assert!(biblioteca.buscar_prestamo(&libro1, &cliente1).is_some());
        assert!(biblioteca.buscar_prestamo(&libro1, &cliente2).is_none());

        // // Probar préstamos a vencer
        let prestamos_proximos = biblioteca.prestamos_a_vencer(10);
        assert_eq!(prestamos_proximos.len(), 5); // 5 préstamos de cliente1

        // Probar préstamos vencidos
        let mut fecha_pasada = Fecha::fecha_actual();
        fecha_pasada.restar_dias(1);
        biblioteca.prestar(&cliente2, &libro1, &fecha_pasada);
        let prestamos_vencidos = biblioteca.prestamos_vencidos();
        assert_eq!(prestamos_vencidos.len(), 1); // Solo el último préstamo está vencido

        // Probar devolver libro
        assert_eq!(biblioteca.cant_disponibles(&libro1), 3);
        assert_eq!(biblioteca.cant_prestamos_cli(&cliente1), 5);
        assert!(biblioteca.devolver_libro(&libro1, &cliente1));
        assert_eq!(biblioteca.cant_disponibles(&libro1), 4); // Se incrementa al devolver
        assert_eq!(biblioteca.cant_prestamos_cli(&cliente1), 4); // Un préstamo menos
        assert!(!biblioteca.devolver_libro(&libro1, &cliente1)); // No se puede devolver dos veces
    }
}

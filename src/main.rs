mod tp02;
mod tp03;

fn main() {
    let mut persona = tp03::ej01::Persona::new(
        "Tao".to_string(),
        Some("1 entre 2 y 3".to_string()),
        23,
    );
    println!("persona.to_string(): {0}", persona.to_string());
    println!("edad: {0}", persona.obtener_edad());
    persona.actualizar_direccion("marte calle 2".to_string());
    println!("nueva dir: {0}", persona.to_string());
}

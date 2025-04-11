use std::io;
fn main() {
    const CADENA:&str = "rustyrust";
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error al leer input");
    let modi:char = input.trim().parse().expect("valor malo, era un char, loco");

    let mut count = 0;
    for char in CADENA.chars() {
        if modi == char {
            count += 1;
        }
    }
    println!("la cant de veces que aparece la letra {modi} en {CADENA} es: {count}");
}

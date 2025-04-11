use std::io;

fn main() {
    let cadenas:[&str; 5] = ["rust", "crab", "tao", "loco", "lesto"];
    println!("cadenas:{cadenas:?}");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error al leer de stdin");
    let usr_string:String = input.trim().parse().expect("no pusiste un string o q onda");

    let mut encontrado = false;
    for i in cadenas {
        if i == usr_string {
            encontrado = true;
        }
    }

    println!("la cadena ingresada {usr_string} se encontró en cadenas?: {encontrado}");
}

use std::io;
fn main() {
    let cadena:String = "hola ".to_string();
    println!("vas a concatenar la cadena:{cadena} ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error al inputear");

    let modi:String = match input.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("valor malo");
            return;
        }
    };

    let res = cadena + &modi;
    println!("resultado:{res}");
}

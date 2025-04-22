use std::io;
fn main() {
    let num:u32 = 3;
    println!("ini num: {num}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error al leer de stdin");

    let modi:u32 = input.trim().parse().expect("valor malo");  
    let resul = (num + modi).pow(2);
    println!("resultado:{resul}");
}

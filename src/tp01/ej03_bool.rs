use std::io;

fn main() {
    let bul:bool = true;
    println!("vas a operar con el valor {bul}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error al leer");

    let modi:bool = match input.trim().parse() {
        Ok(b) => b,
        Err(_) => {
            println!("valor malo");
            return;
        }
    };
    let res_and = bul & modi;
    let res_or = bul | modi;
    println!("and:{} \n or:{}", res_and, res_or);
}

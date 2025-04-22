use std::io;

fn main() {
    let num:f32 = 33.0;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error al leer");
    
    let modifier:f32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("no inputeaste un numerito");
            return;
        }
    };

    println!("num:{}, mod:{}",num,modifier);
    println!("suma:{} \n resta:{} \n mult:{} \n div:{}", num+modifier, num-modifier, num * modifier, num/modifier);
}

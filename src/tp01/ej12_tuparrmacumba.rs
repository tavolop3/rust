fn main() {
    let tup:(&str, [i32;3]) = ("epaa", [1,2,3]);
    let mut tot = 0;
    for i in tup.1 {
        tot += i;
    }
    println!("cadena:{}\ntot:{tot}", tup.0);
}

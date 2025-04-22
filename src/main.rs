mod tp02;

fn main() {
    let num1 = 2;
    let is_even = tp02::ej01::is_even(num1);
    println!("ej1 -> num:{num1}, is even? {is_even}");

    let num2 = 4;
    let is_prime = tp02::ej02::is_prime(num2);
    println!("ej2 -> num:{num2}, is prime? {is_prime}");

    let arr: [u64; 5] = [1, 2, 3, 4, 5];
    let sum = tp02::ej03::sum_arr(arr);
    println!("ej3 -> arr:{arr:?}, sum:{sum}");

    let n_odds = tp02::ej04::n_odds(arr);
    println!("ej4 -> arr:{arr:?}, num odds:{n_odds}");

    let arrf: [f64; 5] = [1.1, 5.5, 3.3, 4.4, 5.5];
    let dup_arr = tp02::ej05::dup_vals(arrf);
    println!("ej5 -> arr:{arrf:?}, new arr:{dup_arr:?}");

    
}

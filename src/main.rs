mod tp02;
mod tp03;

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
    let dup_arr = tp02::ej05::dup_vals_hashmap(arrf);
    println!("ej5 -> arr:{arrf:?}, dups:{dup_arr:?}");

    let strarr = ["hola".to_string(), "na".to_string(), "que".to_string()];
    let longs = tp02::ej06::longitud_de_cadenas(&strarr);
    println!("ej6 -> arr:{strarr:?}, longs:{longs:?}");

    let intarr = [3, 5, 7];
    let limite = 4;
    let cant = tp02::ej07::cantidad_de_mayores(&intarr, limite);
    println!("ej7 -> arr:{intarr:?}, limite:{limite}, cant:{cant}");

    let fnums1 = [1.0, 2.5, 3.0];
    let fnums2 = [2.0, 0.5, 0.0];
    let sums = tp02::ej08::sumar_arreglos(&fnums1, &fnums2);
    println!("ej8 -> arr1:{fnums1:?}, arr2:{fnums2:?}, sumarr:{sums:?}");

    let nums = [1, 2, 3, 4, 5];
    let inf = 2;
    let sup = 4;
    let cant = tp02::ej09::cantidad_en_rango(&nums, inf, sup);
    println!("ej9 -> arr:{arr:?}, inf:{inf}, sup:{sup}, cant:{cant}");

    let strarr = ["epa".to_string(), "mepase".to_string()];
    let lim = 3;
    let cant = tp02::ej10::cantidad_de_cadenas_mayor_a(&strarr, lim);
    println!("ej10 -> arr:{strarr:?}, lim:{lim}, cant:{cant}");

    let original = [1, 2, 3];
    let mut nums = [1, 2, 3];
    let factor = 2;
    tp02::ej11::multiplicar_valores(&mut nums, factor);
    println!("ej11 -> arr:{original:?}, factor:{factor}, arr:{nums:?}");

    let mut nums = [1,2,3];
    tp02::ej12::reemplazar_pares(&mut nums);
    println!("ej12 -> arr:{original:?}, res:{nums:?}");

    let mut arrstrs = ["hola".to_string(), "ah bue".to_string()];
    let original = ["hola".to_string(), "ah bue".to_string()];
    tp02::ej13::ordenar_nombres(&mut arrstrs);
    println!("ej13 -> original:{original:?}, res:{arrstrs:?}");

    let mut num = 5.0;
    print!("ej14 -> num:{num}, ");
    tp02::ej14::incrementar(&mut num);
    println!("res:{num}");

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

#[cfg(test)]
mod tests {
    use crate::tp02;

    #[test]
    fn test_ej01_is_even() {
        assert!(tp02::ej01::is_even(2));
        assert!(tp02::ej01::is_even(4));
        assert!(tp02::ej01::is_even(0));
        assert!(!tp02::ej01::is_even(1));
        assert!(!tp02::ej01::is_even(3));
        assert!(!tp02::ej01::is_even(999));
    }

    #[test]
    fn test_ej02_is_prime() {
        assert!(!tp02::ej02::is_prime(0));
        assert!(!tp02::ej02::is_prime(1));
        assert!(tp02::ej02::is_prime(2));
        assert!(tp02::ej02::is_prime(3));
        assert!(!tp02::ej02::is_prime(4));
        assert!(tp02::ej02::is_prime(5));
        assert!(tp02::ej02::is_prime(7));
        assert!(tp02::ej02::is_prime(11));
        assert!(tp02::ej02::is_prime(13));
        assert!(!tp02::ej02::is_prime(15));
        assert!(tp02::ej02::is_prime(17));
        assert!(tp02::ej02::is_prime(19));
        assert!(tp02::ej02::is_prime(97));
        assert!(!tp02::ej02::is_prime(91));
    }

    #[test]
    fn test_ej03_sum_arr() {
        assert_eq!(tp02::ej03::sum_arr([1, 2, 3, 4, 5]), 15);
        assert_eq!(tp02::ej03::sum_arr([0, 0, 0, 0, 0]), 0);
        assert_eq!(tp02::ej03::sum_arr([10, 20, 30, 40, 50]), 150);
        assert_eq!(tp02::ej03::sum_arr([5, 5, 5, 5, 5]), 25);
    }

    #[test]
    fn test_ej04_n_odds() {
        assert_eq!(tp02::ej04::n_odds([1, 2, 3, 4, 5]), 3);
        assert_eq!(tp02::ej04::n_odds([2, 4, 6, 8, 10]), 0);
        assert_eq!(tp02::ej04::n_odds([1, 3, 5, 7, 9]), 5);
        assert_eq!(tp02::ej04::n_odds([0, 0, 0, 0, 1]), 1);
    }

    #[test]
    fn test_ej05_dup_vals_hashmap() {
        let result = tp02::ej05::dup_vals_hashmap([1.1, 2.2, 1.1, 3.3, 4.4]);
        assert_eq!(result[0], 1.1);
        assert_eq!(result[1], 0.0);
        
        let result = tp02::ej05::dup_vals_hashmap([1.1, 1.1, 1.1, 1.1, 1.1]);
        assert_eq!(result[0], 1.1);
        assert_eq!(result[1], 1.1);
        assert_eq!(result[2], 1.1);
        assert_eq!(result[3], 1.1);
        
        let result = tp02::ej05::dup_vals_hashmap([1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result[0], 0.0);
    }

    #[test]
    fn test_ej06_longitud_de_cadenas() {
        let strings = vec!["hola".to_string(), "mundo".to_string(), "rust".to_string()];
        let result = tp02::ej06::longitud_de_cadenas(&strings);
        assert_eq!(result, vec![4, 5, 4]);
        
        let empty_strings: Vec<String> = vec![];
        let result = tp02::ej06::longitud_de_cadenas(&empty_strings);
        assert_eq!(result, Vec::<usize>::new());
        
        let strings = vec!["".to_string(), "a".to_string(), "ab".to_string()];
        let result = tp02::ej06::longitud_de_cadenas(&strings);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_ej07_cantidad_de_mayores() {
        let nums = [1, 5, 10, 15, 20];
        assert_eq!(tp02::ej07::cantidad_de_mayores(&nums, 10), 2);
        assert_eq!(tp02::ej07::cantidad_de_mayores(&nums, 0), 5);
        assert_eq!(tp02::ej07::cantidad_de_mayores(&nums, 20), 0);
        
        let empty: [u64; 0] = [];
        assert_eq!(tp02::ej07::cantidad_de_mayores(&empty, 5), 0);
    }

    #[test]
    fn test_ej08_sumar_arreglos() {
        let arr1 = [1.0, 2.0, 3.0];
        let arr2 = [4.0, 5.0, 6.0];
        let result = tp02::ej08::sumar_arreglos(&arr1, &arr2);
        assert_eq!(result, vec![5.0, 7.0, 9.0]);
        
        let arr1 = [0.5, 1.5, 2.5];
        let arr2 = [0.5, 0.5, 0.5];
        let result = tp02::ej08::sumar_arreglos(&arr1, &arr2);
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
        
        let empty1: [f64; 0] = [];
        let empty2: [f64; 0] = [];
        let result = tp02::ej08::sumar_arreglos(&empty1, &empty2);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_ej09_cantidad_en_rango() {
        let nums = [1, 5, 10, 15, 20];
        assert_eq!(tp02::ej09::cantidad_en_rango(&nums, 5, 15), 3);
        assert_eq!(tp02::ej09::cantidad_en_rango(&nums, 0, 100), 5);
        assert_eq!(tp02::ej09::cantidad_en_rango(&nums, 50, 100), 0);
        
        let empty: [u64; 0] = [];
        assert_eq!(tp02::ej09::cantidad_en_rango(&empty, 1, 10), 0);
    }

    #[test]
    fn test_ej10_cantidad_de_cadenas_mayor_a() {
        let strings = vec!["a".to_string(), "abc".to_string(), "abcde".to_string()];
        assert_eq!(tp02::ej10::cantidad_de_cadenas_mayor_a(&strings, 2), 2);
        assert_eq!(tp02::ej10::cantidad_de_cadenas_mayor_a(&strings, 0), 3);
        assert_eq!(tp02::ej10::cantidad_de_cadenas_mayor_a(&strings, 5), 0);
        
        let empty: Vec<String> = vec![];
        assert_eq!(tp02::ej10::cantidad_de_cadenas_mayor_a(&empty, 3), 0);
    }

    #[test]
    fn test_ej11_multiplicar_valores() {
        let mut nums = [1, 2, 3];
        tp02::ej11::multiplicar_valores(&mut nums, 2);
        assert_eq!(nums, [2, 4, 6]);
        
        let mut nums = [5, 10, 15];
        tp02::ej11::multiplicar_valores(&mut nums, 0);
        assert_eq!(nums, [0, 0, 0]);
        
        let mut empty: [u64; 0] = [];
        tp02::ej11::multiplicar_valores(&mut empty, 10);
        assert_eq!(empty, []);
    }

    #[test]
    fn test_ej12_reemplazar_pares() {
        let mut nums = [1, 2, 3, 4, 5];
        tp02::ej12::reemplazar_pares(&mut nums);
        assert_eq!(nums, [1, -1, 3, -1, 5]);
        
        let mut nums = [2, 4, 6];
        tp02::ej12::reemplazar_pares(&mut nums);
        assert_eq!(nums, [-1, -1, -1]);
        
        let mut nums = [1, 3, 5];
        tp02::ej12::reemplazar_pares(&mut nums);
        assert_eq!(nums, [1, 3, 5]);
        
        let mut empty: [i64; 0] = [];
        tp02::ej12::reemplazar_pares(&mut empty);
        assert_eq!(empty, []);
    }

    #[test]
    fn test_ej13_ordenar_nombres() {
        let mut names = vec!["zebra".to_string(), "apple".to_string(), "banana".to_string()];
        tp02::ej13::ordenar_nombres(&mut names);
        assert_eq!(names, vec!["apple", "banana", "zebra"]);
        
        let mut single = vec!["single".to_string()];
        tp02::ej13::ordenar_nombres(&mut single);
        assert_eq!(single, vec!["single"]);
        
        let mut empty: Vec<String> = vec![];
        tp02::ej13::ordenar_nombres(&mut empty);
        assert_eq!(empty, Vec::<String>::new());
    }

    #[test]
    fn test_ej14_incrementar() {
        let mut num = 5.0;
        tp02::ej14::incrementar(&mut num);
        assert_eq!(num, 6.0);
        
        let mut num = 0.0;
        tp02::ej14::incrementar(&mut num);
        assert_eq!(num, 1.0);
        
        let mut num = -1.0;
        tp02::ej14::incrementar(&mut num);
        assert_eq!(num, 0.0);
    }
}

#![allow(unused_variables, dead_code)]
// Escriba una función que reciba un vector de números enteros y retorna la cantidad de
// números primos. Cree un trait para la determinación del número primo e impleméntelo
// según corresponda. Utilice la función iter sobre el vector y aplique un closure para
// resolverlo.

pub fn cant_primos(nums: Vec<i32>) -> usize {
    nums.iter().filter(|&n| n.es_primo()).count()
}

trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        if *self <= 1 {
            return false;
        }

        if *self == 2 {
            return true;
        }

        if *self % 2 == 0 {
            return false;
        }

        let lim = (*self as f64).sqrt() as u64;
        for i in (3..=lim).step_by(2) {
            if *self % i as i32 == 0 {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_ej01_esprimo() {
    let nums: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert!(cant_primos(nums) == 4);

    let nums: Vec<i32> = vec![0, 1, 4, 6];
    assert!(cant_primos(nums) == 0);

    let nums: Vec<i32> = vec![];
    assert!(cant_primos(nums) == 0);
}

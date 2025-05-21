#![allow(dead_code, unused_variables)]

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let lim = (n as f64).sqrt() as u64;
    for i in (3..=lim).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

/*
Explicación del algoritmo:

    Para optimizar, solo comprobamos divisores hasta la raíz cuadrada del número.
    Solo verificamos divisores impares (empezando desde 3) para mejorar la eficiencia.
    Si encontramos algún divisor, retornamos false.
    Si no encontramos divisores, retornamos true.
*/

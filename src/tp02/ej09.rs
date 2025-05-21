#![allow(dead_code, unused_variables)]

// Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
// enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
// función retorna la cantidad de números del arreglo que están entre el rango de los
// parámetros inferior y superior inclusive.

pub fn cantidad_en_rango(arr: &[u64], inf: u64, sup: u64) -> u64 {
    let mut cant = 0;

    // el patrón &i automáticamente desreferencia la referencia para darte el valor directamente, sin necesidad de escribir *i explícitamente.
    for &i in arr {
        if i >= inf && i <= sup {
            cant += 1;
        }
    }

    return cant;
}

#![allow(dead_code, unused_variables)]

// Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
// enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
// por el parámetro factor modificándolo.

pub fn multiplicar_valores(arr: &mut [u64], factor: u64) {
    for i in 0..arr.len() {
        arr[i] = arr[i] * factor;
    }
}

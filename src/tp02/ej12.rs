#![allow(dead_code, unused_variables)]

// Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
// reemplaza todos los números pares por -1.

pub fn reemplazar_pares(arr: &mut [i64]) {
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            arr[i] = -1;
        }
    }
}

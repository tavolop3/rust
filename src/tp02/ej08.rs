// Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
// números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
// arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
// arreglos pasados por parámetro.

pub fn sumar_arreglos(arr1:&[f64], arr2:&[f64]) -> Vec<f64> {
    let mut sums = Vec::new();

    for i in 0..arr1.len() {
        sums.push(arr1[i] + arr2[i]);
    }

    return sums;
}

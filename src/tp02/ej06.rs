#![allow(dead_code, unused_variables)]

// Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
// un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
// arreglo.

// &[f64] slice array
pub fn longitud_de_cadenas(arr: &[String]) -> Vec<usize> {
    let mut longs = Vec::new();

    for i in 0..arr.len() {
        longs.push(arr[i].len());
    }

    return longs;
}

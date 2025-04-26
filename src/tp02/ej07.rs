// Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
// de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
// números mayores al límite que tiene el arreglo.

pub fn cantidad_de_mayores(arr:&[u64], limite:u64) -> u64 {
    let mut cant = 0;

    for i in arr {
        if *i > limite {
            cant += 1;
        }
    }

    return cant;
}

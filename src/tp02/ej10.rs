// Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
// un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
// del arreglo que son de longitud mayor al parámetro límite.

pub fn cantidad_de_cadenas_mayor_a(arr:&[String], lim:usize) -> u64 {
    let mut cant = 0;

    for i in arr {
        if i.len() > lim {
            cant += 1;
        }
    }

    return cant;
}

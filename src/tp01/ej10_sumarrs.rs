fn main() {
    let arr1 = [1,2,3,4,5];
    let arr2 = [4,3,2,1,0];
    let mut arr_tot = [0;5];

    for i in 0..arr_tot.len() {
        arr_tot[i] = arr1[i] + arr2[i];
    }
    println!("arr_tot:{arr_tot:?}");
}

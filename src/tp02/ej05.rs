pub fn dup_vals(arr: [f64; 5]) -> [f64;5] {
    let mut dups:[f64; 5] = [0.0;5];
    let mut dup_i = 0;
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] == arr[j] {
                dups[dup_i] = arr[i];
                dup_i += 1;
            }
        }
    }

    return dups;
}

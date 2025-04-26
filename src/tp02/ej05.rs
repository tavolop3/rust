use std::collections::HashMap;

// pub fn dup_vals(arr: [f64; 5]) -> [f64;5] {
//     let mut dups:[f64; 5] = [0.0;5];
//     let mut dup_i = 0;
//     for i in 0..arr.len() {
//         for j in i+1..arr.len() {
//             if arr[i] == arr[j] {
//                 dups[dup_i] = arr[i];
//                 dup_i += 1;
//             }
//         }
//     }
//
//     return dups;
// }

pub fn dup_vals_hashmap(arr: [f64;5]) -> [f64;5] {
    let mut dups = [0.0;5];
    let mut nums = HashMap::new();
    let mut dup_i = 0;

    for i in arr {
        if nums.contains_key(&i.to_string()) {
            dups[dup_i] = i;
            dup_i += 1;
        } else {
            nums.insert(i.to_string(), true);
        }
    }

    return dups;
}

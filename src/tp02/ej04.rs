use crate::tp02::ej01::is_even;

pub fn n_odds(arr: [u64; 5]) -> u64 {
    let mut tot = 0;
    for i in arr {
        if !is_even(i) {
            tot += 1;
        }
    }
    return tot;
}

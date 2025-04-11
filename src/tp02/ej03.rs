pub fn sum_arr(arr: [u64; 5]) -> u64 {
    let mut tot = 0;
    for i in arr {
        tot += i;
    }
    return tot;
}

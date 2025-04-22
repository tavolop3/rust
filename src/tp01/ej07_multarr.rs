fn main() {
    const MULTIPLIER:i8 = 2;
    let mut arr = [1,2,3,4,5,6];
    println!("MULTIPLIER: {MULTIPLIER} \narr:{:?}", arr);
    for i in 0..5 {
        arr[i] = arr[i] * MULTIPLIER;
    }
    println!("res:{:?}", arr);
}

fn main() {
    // multi dimensional arrays
    let mut arr1 = [[0; 10]; 10];

    for i in 0..arr1.len() {
        for j in 0..arr1[i].len() {
            print!("{} ", arr1[i][j]);
        }
        println!();
    }
}

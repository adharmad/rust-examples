fn main() {
    // arrays
    let arr1 = ["This", "is", "a", "test"];
    println!("{}-{}-{}-{}", arr1[0], arr1[1], arr1[2], arr1[3]);

    // array length
    let arr2 = [1, 2, 3, 4, 5];
    println!("Length of arr2 = {}", arr2.len());

    // mutable array
    let mut arr3 = [1, 2, 3, 4, 5];
    for i in 1..arr3.len() {
        arr3[i] = arr3[i] * arr3[i];
    }

    println!("{}-{}-{}-{}-{}", arr3[0], arr3[1], arr3[2], arr3[3], arr3[4]);

}

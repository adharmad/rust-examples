fn main() {
    // define array of strings having length 10
    let mut arr1 = ["hello"; 10];
    arr1[5] = "mellow";

    println!("{} {}", arr1[0], arr1[5]);
}

fn main() {
    let x = 46; // immutable
    println!("{} = {}", "Value of x ", x);

    let mut y = 33; // mutable
    println!("y = {}", y);

    y = 43; // mutable
    println!("Now y = {}", y);

    let a = 12; // unused variable will raise a warning
    let _b = 13; // unused variable will NOT raise a warning
}

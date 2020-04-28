fn main() {
    // integer arithmetic
    let a = 50;
    let b = 10;

    println!("Sum of {} and {} = {}", a, b, a+b);
    println!("Difference between {} and {} = {}", a, b, a-b);
    println!("Product of {} and {} = {}", a, b, a*b);
    println!("Division of {} and {} = {}", a, b, a/b);
    
    // floating point arithmetic
    let p = 123.456;
    let q = 3.1415;

    println!("{} + {} = {}", p, q, p+q);
    println!("{} - {} = {}", p, q, p-q);
    println!("{} * {} = {}", p, q, p*q);
    println!("{} / {} = {}", p, q, p/q);
}

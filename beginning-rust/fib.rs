fn main() {
    let mut fib = [1; 40];

    for i in 2..fib.len() {
        fib[i] = fib[i-1] + fib[i-2];
    }

    for i in 0..fib.len() {
        println!("fib[{}] = {}", i, fib[i]);
    }

}

fn main() {
    // while loop
    println!("Printing numbers and their squares");
    let mut i = 1;
    while i <= 10 {
        println!("{}, {}", i, i*i);
        i += 1;
    }

    println!("Printing numbers and their cubes (for numbers not divisible by 3)");
    let mut i = 1;
    while i <= 100 {
        if i%3 != 0 {
            println!("{}, {}", i, i*i*i);
        }
        i += 1;
    }
}

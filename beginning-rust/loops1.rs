fn main() {
    let mut i = 1;
    // infinite loop
    // can also be implemented using:
    //while true {
    loop {
        let ii = i*i;
        println!("{} ^2 = {}", i, ii);
        if ii >= 256 {
            break;
        }
        i += 1;
    }

    // for loops
    let mut sum = 0;
    for i in 1..50 {
        sum += i;
        println!("Sum of all numbers until {} = {}", i, sum);
    }
}

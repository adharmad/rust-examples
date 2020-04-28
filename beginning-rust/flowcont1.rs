fn main() {
    let num = -1;

    if num > 0 {
        println!("Number {} is positive", num);
    } else if num < 0 {
        println!("Number {} is negative", num);
    } else if num == 0 {
        println!("Number {} is zero", num);
    }

    // Conditional expressions can be used in print statements
    // like the below
    println!("This number is {}", 
        if num > 0 {
            "positive"
        } else if num < 0 {
            "negative"
        } else {
            "zero"
        }
    );

    let num1 = 100;
    let a = if num1 == 100 { "hundred" } else { "not hundred" };

    println!("a = {}", a);

}

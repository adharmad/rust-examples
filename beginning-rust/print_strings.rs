fn main() {
    // Formatted print string
    print!("{}, {}!\n", "Hello", "World");

    // multi line print
    print!("This is a test\nSecond line\nThird line\n");

    // print with end-of-line
    println!("This prints an EOL");

    // formatted print an int
    println!("{} {}", "My age is", 100);

    // printing a multi line literal string
    println!("This is a 
        string that is multi-line
        third line
        and fourth line");

    // long string on multiple lines for readability
    println!("This is a very long string \
        that is just on multiple lines in the source code \
        for readability purposes");
}

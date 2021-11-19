pub fn run() {
    //print to console
    println!("Hello from the print.rs file.");

    //basic formatting
    println!("Number: {}, Letter: {}", 1, "A");

    //positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "Code"
    );

    //named arguments
    println!(
        "{name} likes to play {sport}.",
        name = "John",
        sport = "football"
    );

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

    //Placeholder fordebug trair
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}

pub fn run() {
    let name = "Jonathan";
    let mut age = 37;
    println!("My name is {}, and I am {}", name, age);
    age = 38;
    println!("My name is {}, and I am {}", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Jonathan", 37);
    println!("{} is {}", my_name, my_age);
}

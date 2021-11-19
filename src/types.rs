pub fn run() {
    let x = 1;
    let y = 2.5;

    let z: i64 = 324234324334;

    println!("Mac i32: {}", std::i32::MAX);
    println!("Mac i64: {}", std::i64::MAX);

    let is_active: bool = true;

    let is_greater = 10 > 1;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}

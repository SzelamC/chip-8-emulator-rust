fn main() {
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");

    let op: u16 = 0x0000;
    let x: u16 = 0x0001;
    match (op, x) {
        (1, 2) => {}
        (1, 3) => {}
        (1, 1) => {}
        (_, _) => return,
    }
}

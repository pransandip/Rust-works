fn type_convertion() {
    let m = 127_000 as i64;
    let n = 20_i32;

    let p = m / (n as i64);
    println!("{}", p);
}

fn main() {
    // let x: u8 = 9; // 0 to 255
    // let y: i8 = 10; // -128 to 127
    // let z = x + y;
    // println!("{}", z);

    let x = 9.5f32;
    let y = 10.5f32;
    let z = x + y;
    println!("{}", z);

    println!("-------typeConvertion-------");
    type_convertion();
}

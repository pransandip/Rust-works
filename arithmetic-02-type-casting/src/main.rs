// Bitwise operation
fn bitwise_operation() {
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101u32);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101u32);
}

fn main() {
    // integer addition
    assert!(1u32 + 2u32 == 3u32);

    // integer substraction
    assert!(1i32 - 2_i32 == -1i32);
    assert!(1i8 - 2i8 == -1_i8);

    assert!(3 * 50 == 150); // i32

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);

    assert!(24_i8 % 5_i8 == 4_i8);

    bitwise_operation();
}

/*
* u8 -->  0 - 255 : if you exceed this number it will throw an error
* i8 --> -128  -  +127
* you can't add two different type of integers it wil throw an error
*/
fn main() {
    let x: f32 = 255.0; // let x = 255.0f32 ;
    let y: f32 = 10.0; // let y = 10.0_f32;
    let k = 127_0000 as i64;

    let m = (i32::MAX as i64) + 1;
    let n = 10_i32;

    let z = x / y;
    let v = k + (y as i64);

    let j = m as i32 / n; //

    println!("z: {}", z);
    println!("v: {}", v);

    println!("j: {}", j);
}

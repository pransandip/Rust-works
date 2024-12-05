/*
* Unsigned int can't be negative
* By default int in rust is 32 bit
* u8: 0 to 2^8 - 1 = 0 to 255 range of numbers you can represent
* i8: -2^7 to 2^7 - 1 = -128 to 127 range of numbers you can represent
* By default float in rust is 64 bit
* scaler and compound datatypes
* tuple is a fixed lenght sequence of element that is immutable
*/

fn arrays() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 6;
    println!("{}", arr[4]);
}

fn main() {
    let floating_point: f32 = 10.39;
    let true_or_false = true;
    let letter = 'a';
    println!("{}, {}, {}", floating_point, true_or_false, letter);

    let mut tup = (1, true, 'a');
    tup.0 = 10;
    println!("{}", tup.0);

    arrays();
}

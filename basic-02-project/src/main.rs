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

    let byte = [0; 8]; // length of arr is 8 and all elements are 0
    println!("2nd element of byte array is: {}", byte[1]);
}

fn datatypes() {
    let letter = 'a';
    let true_or_false = true;
    let floating_point: f32 = 10.39;
    println!("{}, {}, {}", letter, !true_or_false, floating_point);
}

fn tuples() {
    let mut tup1: (i32, bool, char) = (1, true, 'a');
    tup1.0 = 10;
    println!("{}", tup1.0);

    let tup2: (&str, u8) = ("Sandip Roy", 56);
    let (name, weight) = tup2;

    println!("name is: {} and weight: {}kg", name, weight)
}

fn main() {
    arrays();
    tuples();
    datatypes();
}

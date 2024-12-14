/*
* Rust has scaler and compound datatypes
* By default int in rust is i32 bit
* By default float in rust is f64 bit
* u8: 0 to 2^8 - 1 = 0 to 255 range of numbers you can represent
* i8: -2^7 to 2^7 - 1 = -128 to 127 range of numbers you can represent
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

// tuple is a fixed lenght sequence of element that is immutable
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

    //Type Inference
    let my_num = 1_______0_______8__________u8;
    println!("my_num: {}", my_num)
}

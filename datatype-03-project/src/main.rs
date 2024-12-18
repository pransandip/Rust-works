/*
* Primitive dataTypes: Basic/Fundamental data type used to declare a value
* Primitive data types has two category: 1.) Scalar type 2.) Compound type
* Scalar: A scalar type represents a single value. Rust has four primary
* scalar types: integers, floating-point numbers, Booleans, and characters.
* Compound: compound types can group multiple values into one type.
* Rust has two primitive compound types: tuples and arrays.
*/

/*
* Tuple is fixed length sequence of elements that is imutable
* Arrays have to have same element inside
*/

fn scalar_types() {
    let x: i16 = 25;
    let y: u16 = 56;
    let z = 10.9;
    let true_or_false = true;
    let letter = '@';
    const SECONDS: u32 = 60;
    let charecter: char = ';';
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("true_or_false is: {}", true_or_false);
    println!("letter is: {}", letter);
    println!("SECONDS: {}", SECONDS);
    println!("char is: {}", charecter);
}

fn compound_types() {
    let mut tup = (1, true, 's');
    tup.0 = 123;
    println!("tup(0): {}", tup.0);

    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 144;
    println!("arr[0]: {}", arr[0]);
}

fn type_casting() {
    // when we initialize x to y we cannot convert datatype u8 to other.
    let x: u8 = 4;
    let y = x;
    println!("x: {}, y:{}", x, y);
}

fn main() {
    println!("/*----scalar_types----*/");
    scalar_types();
    println!("/*----compound_types----*/");
    compound_types();
    println!("/*----type_casting----*/");
    type_casting();
}

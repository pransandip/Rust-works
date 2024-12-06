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
*/

/*
 * arrays have to have same element inside
 */

fn data_types() {
    const SECONDS: u32 = 60;
    println!("SECONDS: {}", SECONDS);

    let charecter: char = ';';
    println!("char is: {}", charecter);

    let mut tup: (i8, bool, char) = (1, true, 's');
    tup.0 = 100;
    println!("{}", tup.0);

    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 5;
    println!("{}", arr[0]);
}

fn main() {
    data_types();
}

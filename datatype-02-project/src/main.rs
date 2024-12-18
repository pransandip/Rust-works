/*
* Primitive dataTypes: Basic/Fundamental data type used to declare a value
* Primitive data types has two category: 1.) Scalar type 2.) Compound type
* Scalar: A scalar type represents a single value. Rust has four primary
* scalar types: integers, floating-point numbers, Booleans, and characters.
* Compound: compound types can group multiple values into one type.
* Rust has two primitive compound types: tuples and arrays.
*/

// 1. Tuple is fixed length sequence of elements that is imutable
// 2. arrays have to have same element inside

/*
 * fn
 * () nothing is going on
 * ! = macro = super function, functions that write code
*/

fn main() {
    // Note this is a print! not println!
    print!("\t'Start with a tab'\n'and move to a new line' \n");
    println!(r#"He said, "you can find the file at c:\tab\file.txt." then i found the file"#);
    println!(r##"He said, "#sandy you are a champ" then i got the trophy."##);
    println!("{1} {0}", "Sandy", "Roy");
    println!(
        "{city1} is in {country} and {city2} is also in {country} but {city3} is not in {country}.",
        city1 = "Bangalore",
        city2 = "Kolkata",
        city3 = "Tokyo",
        country = "India",
    );
    println!("{:?}", b"This"); // turn strings into bytes

    for byte in "This".as_bytes() {
        print!("{},", byte);
    }

    // cast char as u16 to get the hexadecimal value
    println!("\n{:X}", '₥' as u16);
    println!("{:X}", '₩' as u16);
    println!("{:X}", '⨈' as u16);
    println!("{:X}", '₹' as u16);

    println!("\u{20A5}, \u{20A9}, \u{2A08},\u{20B9}"); // Try printing them with unicode escape \u

    let number = 555;
    println!(
        "Binary: {:b}, hexadecimal: {:x}, octal: {:o}",
        number, number, number
    );
}

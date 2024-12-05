/*
* rust program auto imports preludes like
* fn, println! main() from rust std library
*/

use std::io;

fn take_input() {
    println!("please input something!");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("Hello! {}", input);
}

// converting string to integer
fn string_to_integer() {
    println!("please enter a number!");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 3);
}

fn main() {
    take_input();
    string_to_integer();
}

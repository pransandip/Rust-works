/*
* rust program auto imports preludes like
* fn, println! from rust std library
*/

use std::io;

fn main() {
    println!("please input something!");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    println!("{}", input);
}

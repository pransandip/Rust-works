/*
TODO: The Rust Prelude
* Rust comes with a variety of things in its standard library. However,
* if you had to manually import every single thing that you used, it would
* be very verbose. But importing a lot of things that a program never uses
* isn’t good either. A balance needs to be struck.

! The prelude is the list of things that Rust automatically imports into every
* Rust program. It’s kept as small as possible, and is focused on things,
* particularly traits, which are used in almost every single Rust program.

* read_line() will return a result object to handle it you need expect().
*/

use std::io;

fn main() {
    println!("Enter something: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed t read line");
    println!("You Entered: {}", input);
}

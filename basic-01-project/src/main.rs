/*
* You have explicit and implicit type
* name Shadowing
* variables are immutable
* const can not be redefine
*/

fn mutable() {
    let mut x = 4;
    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x);
}

fn redeclare() {
    let y = 8;
    println!("y is: {}", y);
    {
        // scope changed
        let y = y - 2;
        println!("y is: {}", y);
    }
    let y = y + 2;
    println!("y is: {}", y);
}

fn constants() {
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}

fn main() {
    println!("----mutable---");
    mutable();
    println!("----redeclare----");
    redeclare();
    println!("----constants---");
    constants();
}

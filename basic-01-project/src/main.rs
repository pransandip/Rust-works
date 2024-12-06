/*
* You have four spaces before println
* println!() macro takes strings
* You have explicit and implicit type
* name shadowing
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

    // overflow
    let mut x: i8 = 10;

    // If you have run time logic compliler not going to check it
    for _i in 0..1000 {
        x = x + 100;
    }

    println!("x = {}", x)
}

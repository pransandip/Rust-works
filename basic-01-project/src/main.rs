/*
 * You have four spaces before println!()
 * You have explicit and implicit type
 * variables are immutable
 * const can't be redefine
 */

/* Shadowing: when you shadow a variable, you don't destroy it. you block it. */

/* only u8 as a cast as a 'char', not i32 */
/* usize = 64 bits *if possible* - if not, 32 bit */

fn mutable() {
    let mut x = 4;
    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x as u8 as char); // double cast
}

fn redeclare() {
    let y = 222;
    println!("y is: {}", y);
    {
        // scope changed
        let y = y - 2;
        println!("y is: {}", y);
    }
    let y = y + 2;
    println!("y is: {}", y as u8);
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

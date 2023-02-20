/*
* You have explicit and implicit type
* name Shadowing
* variables are immutable
* const can not be redefine
*/

fn data_types() {
    let x = 56;
    println!("x is: {}", x);

    {
        let x = 2;
        println!("x is: {}", x);
    }

    let x = x + 1;
    println!("x is: {}", x);

    let x = "hello";
    println!("x is: {}", x);
}

fn print_time() {
    const SECONDS: u32 = 60;
    println!("SECONDS: {}", SECONDS)
}

fn main() {
    data_types();
    print_time();
}

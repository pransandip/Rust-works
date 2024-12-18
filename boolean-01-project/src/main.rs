fn main() {
    check_male();
    control_flow();
    check_odd_even();
    check_true_false();
}

// conditionals
fn control_flow() {
    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 };
    println!("{}", number);
}

fn check_true_false() {
    let is_even: bool = false;

    if is_even {
        println!("The Number is even");
    } else if !is_even {
        println!("The Number is odd");
    }
}

fn check_male() {
    let is_male: bool = true;
    let is_above_18: bool = false;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male")
    }
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn check_odd_even() {
    let x: i32 = 99;
    let is_even = is_even(x);

    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
}

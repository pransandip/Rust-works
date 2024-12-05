fn main() {
    check_male();
    check_odd_even();
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

fn check_odd_even() {
    let x: i32 = 99;
    let is_even = is_even(x);

    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

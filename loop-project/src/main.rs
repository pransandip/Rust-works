use std::ops::{Range, RangeInclusive};
fn main() {
    for i in 0..10 {
        println!("{}", i);
    }
    check_odd_even();
    print_char();
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("success!");
}

// print char in decimal format
fn print_char() {
    let mut sum: i32 = 0;

    // print -3 to 2 where 2 is exclusive
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);
    // print a to z where z is inclusive
    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

// conditionals,loops
fn check_odd_even() {
    let is_even: bool = false;

    if is_even {
        println!("The Number is even");
    } else if !is_even {
        println!("The Number is odd");
    }
}

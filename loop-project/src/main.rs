use std::ops::{Range, RangeInclusive};

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

fn main() {
    print_char();
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("success!");
}

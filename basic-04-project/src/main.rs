/*
* Function, Expressions and Statement
*/

// expression
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // expression
    let number = {
        let x = 3;
        x + 1
    };
    println!("number: {}", number);

    let result = add_numbers(20, 30);
    println!("result: {}", result);
}
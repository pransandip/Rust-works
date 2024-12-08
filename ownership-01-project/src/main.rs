// ------------ Ownership rules -------------
// 1. Each value in rust has a variable that's called its owner, means one variable one owner.
// 2. There can only be one owner at a time, means variable can't have two owner at the same time.
// 3. When owner goes out of scope, the value will be droped.

fn main() {
    {
        let x: i32 = 5;
        let y: i32 = x; // copy

        // s is not valid here, it's not yet declared
        let s: &str = "Hello"; // s is valid from this point forward
                               // do stuff with s, and s is a string literal it's a fixed in size
                               // string literals directly stores in binary

        let txt1: String = String::from("Sandy!"); // txt1 is String type and dynamic in size stored in heap memory
        let txt2: String = txt1; // Move (not shallow copy) txt1 will be droped
        let txt3: String = txt2.clone();

        println!("{} {} {}", y, s, txt3);
    } // this scope is now over, and s is no longer valid

    {
        let words = String::from("ownership");
        takes_ownership(words);
        // println!("{}", words); // value borrowed here after move
    }

    {
        let x: i32 = 15;
        makes_copy(x);
        println!("{}", x)
    }

    {
        let s1 = gives_ownership();
        let s2: String = String::from("value");
        let s3: String = takes_and_gives_back_ownership(s2);
        println!("{} {}", s1, s3);
    }
    {
        let s1: String = String::from("Sandip!");
        let len: usize = calculate_length(&s1);
        println!("length of {} is {}", s1, len)
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("cola");
    return some_string;
}

// Borrowing
fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

// References
fn calculate_length(s: &String) -> usize {
    let lenght = s.len(); // len() returns length of a string
    lenght
}

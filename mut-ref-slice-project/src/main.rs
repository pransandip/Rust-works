// mutable reference

fn main() {
    mut_ref();
}

fn mut_ref() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(_some_string: &mut String) {}

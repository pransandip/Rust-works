// redeclare
fn redeclare() {
    let x: u8 = 4;
    println!("x is {}", x);
    let x: u8 = 5;
    println!("x is {}", x);
    let x = x + 1;
    println!("x is {}", x);
}

// can easily change type in re definition
fn change_type_redeclare() {
    let k: u8 = 10;
    println!("k is {}", k);

    let k = "hello";
    println!("k is {}", k);
}

/*
 * Name Shadowing:
 * using same variable name from different scope
 */

fn name_shadowing() {
    let y: u8 = 4;
    println!("y is {}", y);
    {
        let y: u8 = 2;
        println!("y is {}", y);
    }
    let y: u8 = y + 1;
    println!("y is {}", y);
}

// Interior and exterior scope
fn scope() {
    let z: u8 = 4;
    println!("z is {}", z);
    {
        let z: u8 = z - 2;
        println!("z is {}", z);
    }
    let z: u8 = z + 2;
    println!("z is {}", z);
}

fn main() {
    redeclare();
    change_type_redeclare();
    name_shadowing();
    scope();
}

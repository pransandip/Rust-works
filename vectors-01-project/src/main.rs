fn main() {
    vec_ex();
    new_vec();
    vec_into();
    vec_cap_ex1();
    vec_cap_ex2();
    vec_cap_ex3();
}

fn vec_ex() {
    let my_vec: Vec<&str> = vec!["one", "two", "three"];
    println!("{:?}", my_vec);

    let vec_of_ten: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("three_to_five: {:?}", three_to_five);
    println!("start_at_two: {:?}", start_at_two);
    println!("end_at_five: {:?}", end_at_five);
    println!("everything: {:?}", everything);
}

fn new_vec() {
    let mut my_vec = Vec::new();

    let name1 = String::from("sandy");
    let name2 = String::from("rome");
    let name3 = String::from("kali");

    my_vec.push(name1);
    my_vec.push(name2);
    my_vec.push(name3);

    println!("{:?}", my_vec);
}

fn vec_into() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 4].into(); // Vec<_> means "choose the Vec type for me"
                                            // Rust will choose Vec<i32>
    println!("{:?}\n{:?}", my_vec, my_vec2);
}

fn vec_cap_ex1() {
    let mut my_vec: Vec<char> = Vec::new();
    println!("{}", my_vec.capacity()); // 0 elements prints 0
    my_vec.push('a');
    println!("{}", my_vec.capacity()); // ['a' _ _ _]
}

fn vec_cap_ex2() {
    let mut my_vec: Vec<char> = Vec::with_capacity(5);
    println!("{}", my_vec.capacity()); // capacity: 5
    my_vec.push('a');
    println!("{}", my_vec.capacity()); // ['a' _ _ _ _]

    for _i in 0..1000 {
        my_vec.push('a');
    }

    println!("{}", my_vec.capacity());
}

fn vec_cap_ex3() {
    let mut my_vec: Vec<char> = Vec::with_capacity(4);
    println!("{}", my_vec.capacity()); // capacity: 5
    my_vec.push('a');
    println!("{}", my_vec.capacity()); // ['a' _ _ _ ]
    my_vec.push('a');
    my_vec.push('a');
    my_vec.push('a');
    println!("{}", my_vec.capacity()); // ['a' 'a 'a' 'a' ]
}

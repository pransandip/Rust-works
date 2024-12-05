fn many_types() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);
    println!("success!")
}

fn check_types() {
    let x: f64 = 1_000.000_1;
    let y: f32 = 0.12;
    let z: f64 = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("{} {}", y, z);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    many_types();
    check_types();

    assert!(0.1 as f32 + 0.2 as f32 == 0.3_f32);
    println!("success!");
}

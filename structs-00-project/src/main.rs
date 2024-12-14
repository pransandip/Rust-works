// STRUCTS: your own type, your own data structure
struct FileDirectory; // Unit Struct

#[derive(Debug)]
struct Color(u8, u8, u8); // Tuple Struct

#[derive(Debug)]
struct SizeAndColor {
    size: u32,
    color: Color,
} // Named Struct

fn main() {
    let _my_directory = FileDirectory;
    let my_colours = Color(50, 60, 0);

    let size_and_color = SizeAndColor {
        size: 150,
        color: my_colours,
    };

    println!("{:?}", size_and_color);
}
/*
 * String literals(or String constant) is a sequence of characters enclosed in double quotation marks.
 * that represents a null-terminated string. The characters within the quotation marks are interpreted
 * exactly as they appear, without any special meaning or processing.
 *
 *
 * The two most used string types in Rust are String and &str.
 *
 * A String is stored as a vector of bytes (Vec<u8>), but guaranteed
 * to always be a valid UTF-8 sequence. String is heap allocated,
 * growable and not null terminated.
 *
 * &str is a slice (&[u8]) that always points to a valid UTF-8 sequence,
 * and can be used to view into a String, just like &[T] is a view into Vec<T>.
 *
 */

/*
 * In Rust, String::new() creates a new, empty String.
 * while String::from() creates a String from an existing string slice (&str)
 * essentially converting a string literal or another string reference into a fully owned String object;
 *
 * use String::new() when you need an initially empty string to add data to later,
 * and String::from() when you want to convert an existing string into a String object.
 *
 */

// Key points:
/*
 * String::new():
 *
 * Creates an empty String.
 * Useful when you want to build a string by adding characters or substrings iteratively.
 * Does not allocate any initial buffer, so can be more efficient for very large strings
 * that may not be fully populated.
 *
 * String::from():
 *
 * Creates a String from an existing string slice (&str).
 * Takes a string literal or another string reference and converts it to a fully owned String
 * Useful when you need to manipulate a string that is already provided.
 *
 */

fn main() {
    let mut empty_string = String::new(); // An empty string

    let my_string = String::from("Hello, World!"); // A string from a literal

    // Adding to an empty string
    empty_string.push_str("Sandy");

    println!("{}", my_string);

    // Accessing characters from 'my_string'
    println!("{}", my_string.chars().nth(4).unwrap());
}

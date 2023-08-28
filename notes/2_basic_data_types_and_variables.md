## Basic Data Types and Variables

```RUST
fn main() {

    // Boolean
    let my_first_bool = true;
    let my_second_bool = true;
    let my_third_bool: bool = true;
    let _my_fourth_bool: bool = true; // The leading underscore (_) indicates that this variable won't be used, suppressing any warnings.

    // Integer
    // Integers have various bit sizes: 8, 16, 32, 64, 128 bits.
    let just_a_number = 0; // It's automatically inferred as i32.
    let days_of_week: i8 = 7;
    let number_of_users: i64 = 128000;
    // These integers above are signed integers. There are also unsigned integers.
    // Use 'u' instead of 'i' for unsigned integers.
    let number_of_tokens: u64 = 10000;

    // Floating Point Number
    // Floats have bit sizes: 32, 64 bits.
    let pi: f32 = 3.14;

    // Character
    let my_char: char = '1';

    // String
    // There are two types of strings in Rust: String and &str.
    // 1- A String is stored as a vector of bytes.
    // 2- &str is a slice (&[u8]) that always points to a valid UTF-8 sequence.
    let message: &str = "Hi, Simon";
    let message2 = "Hi, Simon";
    let message3 = String::from("Hi, Simon");

    // Array
    // array_name [type; length] = []
    let days_of_the_week: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let first_element = days_of_the_week[0]; // "Monday"
    let last_element = days_of_the_week[days_of_the_week.len() - 1]; // "Sunday"

    // Slice
    let slice = &days_of_the_week[1..3]; // Elements from index 1 to 3 (3 exclusive)
    // "Tuesday", "Wednesday"
    let first_element_of_slice = slice[0]; // "Tuesday"

    // Tuples
    // A tuple is a collection of values of different types.
    // Tuples are immutable by default. Once declared, tuple elements cannot be modified.
    let person = ("Alice", 30);
    let name = person.0; // "Alice"
    let age = person.1; // 30

    // Unit Type
    let unit_type = (); // // Unit type has only one possible value, which is also ()

    // Variables
    let num = 5;
    num = 6; // This will give an error because num is immutable by default.

    // In Rust, variables are immutable by default. To modify a value of a variable, it must be defined with 'mut'.
    let mut num = 5;
    num = 6; // Correct
}

```

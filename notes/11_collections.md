## Collections

```RUST
use std::collections::HashMap;

fn main() {

    // Vectors
    // Vectors are resizable arrays that store elements of the same data type.
    // There are two ways of defining a vector.

    // 1
    let numbers = vec![1, 2, 3, 4];

    // 2
    let mut names: Vec<String> = Vec::new();

    // Add items to a vector
    names.push(String::from("Alice")); // [Alice]
    names.push(String::from("Bob")); // [Alice, Bob]
    // push adds at the end of the vector

    let first_name = &names[0];
    let second_name = &names[1];

    println!("First name is {}, second name is {}", first_name, second_name);
    // Output: First name is Alice, second name is Bob

    // Remove item from a vector
    names.pop(); // [Alice]
    // pop removes at the end of the vector

    for number in &numbers {
        println!("The number is {}", number);
    } // Output:
    // The number is 1
    // The number is 2
    // The number is 3
    // The number is 4

    let slice = &numbers[1..3]; // [2, 3]
    println!("The slice is {:?}", slice); // Output: The slice is [2, 3]

    // Strings
    let mut my_string = String::from("my");
    let _second_string = "Second string".to_string();

    my_string.push_str(" string");

    println!("{}", my_string); // Output: my string

    for c in my_string.chars() {
        println!("{}", c);
    } // Output:
    // m
    // y

    // s
    // t
    // r
    // i
    // n
    // g

    for b in my_string.bytes() {
        println!("{}", b);
    } // Output:
    // 109
    // 121
    // 32
    // 115
    // 116
    // 114
    // 105
    // 110
    // 103

    // Hash Maps
    // A hash map is a collection that associates a key with a value.
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 10); // [Alice: 10]
    scores.insert(String::from("Bob"), 20); // [Alice: 10, Bob: 20]

    let alice_score = scores.get(&String::from("Alice"));
    println!("{:?}", alice_score); // Output: Some(10)
    println!("{:?}", scores); // {"Alice": 10, "Bob": 20}

    scores.remove(&String::from("Bob"));

    println!("{:?}", scores); // {"Alice": 10}

    for (key, value) in &scores {
        println!("{} {}", key, value);
    } // Output: Alice 10

}

```
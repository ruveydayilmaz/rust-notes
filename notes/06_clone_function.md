## Clone Function

```RUST
fn main() {
    // The clone function creates a deep copy of a value. 
    // When you call clone on a value, it returns a new value that is an independent instance 
    // of the original value but has the same data. 
    let original_string = String::from("Hello, world!");
    let cloned_string = original_string.clone(); // Creates a deep copy of the original_string
    
    println!("original_string: {}", original_string);
    println!("cloned_string: {}", cloned_string);

    let original_string = String::from("String");
    let modified_string = modified_string(original_string);
    println!("original string: {}", original_string); // original string: String
    println!("modified string: {}", modified_string); // modified string: String modified
}

fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str(" modified");
    cloned_string
}
```

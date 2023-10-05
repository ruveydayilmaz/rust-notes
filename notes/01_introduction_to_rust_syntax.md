## Introduction to Rust Syntax

```RUST
fn main() {
    // You can define a variable similar to JavaScript
    let _message = "Hello world!";

    let _x: i32 = 42; 
    // _x is the name of the variable,
    // i32 is the type of the variable. In this case, it's an integer
    // 42 is the value.

    let _pi: f64 = 3.14;
    // _pi is the name of the variable,
    // f64 is the type of the variable. in this case, it's a float
    // 3.14 is the value.

    let _is_rust_fun: bool = true;
    // _is_rust_fun is the name of the variable,
    // bool is the type of the variable. in this case, it's a bool
    // true is the value.

    let _letter_a: char = 'a';
    // _letter_a is the name of the variable,
    // char is the type of the variable. in this case, it's a char
    // a is the value.

    // Function syntax
    fn _add(x: i32, y: i32) -> i32 { // function definition
        // The definition tells us that this function gets two integer parameters
        // and returns an integer (-> i32 means it'll return an integer)

        // There are two different types of 'return' definitions:
        // 1 - Using the 'return' keyword
        return x + y;

        // 2 - This is also a way to return a value
        x + y // The important part is that you should not add a semicolon
        // This is more popular among Rust developers
    }

    let x = 4;
    // In Rust, it's possible to redeclare a variable. It's called shadowing.

    // if syntax
    if x >= 0 { // if doesnt have any paranteses
        println!("x is not negative") // The exclamation mark(!) means that println! is not a function; it is a macro.
    } else {
        println!("x is negative")
    }

    let mut i = 1; // 'mut' means it's a mutable variable. Other variables defined above are not mutable.

    // While syntax
    while i <= 5 { 
        println!("{}", i); // {} is a placeholder for the variable i. So, it'll print the value of i.
        i += 1;
    }
}

// run `cargo build` to build the project
// run `cargo run` to run the project

```

## Ownership Concept

```RUST
fn main() {
    // Ownership is the core concept. 
    // Imagine moving boxes to a new house. Stack, similar to a car trunk, is quick and for small items. 
    // The Heap, like a moving truck, is slower but accommodates larger things. 
    // Stack stores fixed-size values, such as ints; Heap handles dynamic sizes, like strings. 
    // In Rust, the stack deals with known sizes, while the Heap handles changing sizes. 
    // Variables cannot share ownership at the same time. Transfer ownership to access values, preventing bugs and maintaining efficiency. 
    // Blockchain development also benefits from this principle. Ownership guarantees secure and fast code.

    let s1 = String::from("hello");
    let s2 = s1; // // Ownership of the string is moved from s1 to s2.

    println!("value of s1 is {}", s1); // This will get an error because the ownership is now in s2.
    // s1 no longer owns the string. Attempting to use s1 would result in a compilation error. 

    let x: i32 = 5;
    let y = String::from("patika");
    let z = y; // Ownership of the string is moved from y to z.
    println!("value of x is {}, and value of z is {}", x, z); // value of x is 5, and value of z is patika
    println!("value of x is {}, and value of y is {}", x, y); // This would give an error.
    // Similar to the first case, the ownership of the string in y has been moved to z.
    // Therefore, attempting to use y after this point would result in a compilation error.

}
```

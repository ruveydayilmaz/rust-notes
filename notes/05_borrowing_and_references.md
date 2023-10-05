## Borrowing and References

```RUST
fn main() {

    // Sometimes, you may need to access a value without owning it.

    // Borrowing
    // Borrowing is a way to temporarily access a value without owning it.
    // It involves creating a reference to the value which enables
    // reading or modifying it without becoming its owner.
    let my_string = String::from("Hello world");
    let my_ref = &my_string;

    println!("My reference is {}", my_ref); // My reference is Hello world

    let my_string = String::from("Hello world");
    print_string(&my_string);

    println!("I still got my string {}", my_ref);

    // Mutable References
    // They are mutable so they can change the variable through these references.
    let mut my_string = String::from("hello");
    change_string(&mut my_string);
    println!("{}", my_string); // hello world

    // multiple immputable references
    let first_immutable_reference = &my_string;
    let second_immutable_reference = &my_string;

    println!("First {}, second {}", first_immutable_reference, second_immutable_reference);
    // First hello world, second hello world

    let first_mutable_reference = &mut my_string;
    println!("First mutable {}", first_mutable_reference);
    // First  mutable hello world

    println!("Immutable {}", first_immutable_reference); // This will throw an error because,
    // after a mutable reference, the immutable reference will give error. 
    // They can't be used at the same time

    // If we create a second mutable reference and run the project, the project will work fine.
    // But if we try to print the first mutable reference after this, it will give error.

    // RULE 1 : You can have either one mutable reference or any number of immutable reference to a variable at a time.
    // RULE 2 : You can not have both mutable and immutable reference to the same variable at the same time.
    // RULE 3 : References must always be valid and point to a valid memory location.
    // RULE 4 : References automatically expire at the end of their scope.

    // Dangling References
    // A dangling reference is a reference that points to a memory location that has been deallocated, 
    // causing unexpected behavior or a runtime error. 
    // This can happen when a reference is still in scope after the object it refers to has been dropped.

    let new_string = String::from("new string");
    let new_string_ref = return_reference(&new_string);
    println!("new string {}", new_string_ref); // new string new string

    let newer_string = new_string; // ownership moved to newer_string
    // new_string_ref is now deallocated.
    println!("new string reference {}", new_string_ref); // error
}

fn print_string(s: &String) {
    println!("{}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}

fn return_reference(some_string: &String) -> &String {
    some_string
} // some_string goes out of scope, but reference_to_s still points to the memory location
```

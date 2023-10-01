## panic! macro

```RUST
fn main() {
    // When the Rust compiler encounters a situation that it absolutely doesn't know 
    // how to handle or wasn't expecting, it pulls the panic! alarm. 
    // This means it stops executing the program right then and there, 
    // providing an error message to let you know what went horribly wrong.

    let v = vec![1, 2, 3];
    v[99]; // This line will cause a panic!

    panic!("This is where we hit the wall!");

    let vegetables = ["carrot", "tomato", "potato"];

    chop(vegetables[1]); // Don't know how to handle a tomato!
}

fn chop(vegetable: &str) {
    match vegetable {
        "carrot" => println!("Chopping a carrot."),
        "celery" => println!("Chopping a celery."),
        "tomato" => panic!("Don't know how to handle a tomato!"),
        _ => println!("Chopping some unknown vegetable."),
    }
}
```
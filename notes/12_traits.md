## Traits

```RUST
fn main() {
    // Traits
    // Traits define a set of methods that a type must implement to satisfy the trait. 
    // However, unlike interfaces, traits can also provide default implementations for some 
    // or all of their methods, allowing for greater flexibility and code reuse.

    let dog = Dog {
        name: "Rudolf".to_string(),
    };

    dog.speak(); // Output: Rudolf says woof!

    let original_string = String::from("This is the original string");
    let cloned_string = original_string.display();

    println!("{}", cloned_string); // Output: This is the original string

    animal_speaks(&dog); // Output: Rudolf says woof!

    // Inheritance

    let cat = Cat;
    cat.make_sound(); // Output: Meow
    cat.walk(); // Output: The cat is walking

    
}

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

imp Speak for Dog {
    fn speak(&self) {
        println!("{} says woof!", self.name);
    }
}

trait Display {
    fn display(&self) -> String;
}

imp Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}

fn animal_speaks<T: Speak>(animal: &T) {
    animal.speak();
}

trait Animal {
    fn make_sound(&self);

    fn sleep(&self) { // Default method implementation
        println!("Animals sleep");
    }
}

trait Mammal: Animal { // Inheritance
    fn walk(&self);
}

trait Bird: Animal {
    fn fly(&self);
}

struct Cat;

imp Animal for Cat {
    fn make_sound(&self) {
        println!("Meow");
    }
}

imp Mammal for Cat {
    fn walk(&self) {
        println!("The cat is walking");
    }
}
```
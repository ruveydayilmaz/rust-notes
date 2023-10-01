## Trait Objects and Box Pointer

```RUST
fn main() {
    // Box Pointer
    // Box pointers are a type of smart pointer provided by the standard library. 
    // A box is a way to allocate memory on the heap. A Box<T> is a pointer to a value 
    // of type T stored on the heap. It's a bit like a dynamic array, but for a single value.

    let mut pointer = Box::new(5); // reference
    println!("{}", pointer); // Output: 5

    pointer = 10; // Error! It should have the pointer structure but we're giving an integer
    *pointer = 10; // * gets the value of the reference, so this a correct way to change the value of the `pointer`
    println!("{}", pointer); // Output: 10
    
    {
        let temp = Box::new("Simon");
    }

    println!("{}", temp); // Error! We cannot use the pointer outside of a scope

    // Rules for Object Safety
    // 1. The trait must not have any associated constants. 
    // This is because trait objects cannot store any additional state and thus cannot represent constant values.
    // 2. All methods in the trait must meet the following criteria: 
        // a. The method must have a self parameter, either &self, &mut self, or self. 
        // This is because trait objects work with references and cannot be passed by value. 
        // b. The method must not have any type parameters, i.e., it must be free of generic types. 
        // This is because trait objects work with dynamic dispatch and cannot store type information at runtime.

    let my_trait_object: Box<dyn MakeNoise> = Box::new(Bird {
        name: "Tweety".to_string(),
        color: "yellow".to_string(),
    });

    my_trait_object.talk(); // Output: The bird is talking

    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();
    speakers.push(Box::new(Bird {name: "bird1".to_string(), color: "yellow".to_string()}));
    speakers.push(Box::new(Dog {name: "dog1".to_string(), breed: "golden".to_string()}));

    for speaker in speakers {
        speaker.talk();
    }

    // Output:
    // The bird is talking
    // The dog is talking
}

trait MakeNoise {
    fn talk(&self);
}

struct Bird {
    name: String,
    color: String,
}

struct Dog {
    name: String,
    breed: String,
}

imp MakeNoise for Bird {
    talk(&self) {
        println!("The bird is talking");
    }
}

imp MakeNoise for Dog {
    talk(&self) {
        println!("The dog is talking");
    }
}

fn invite_to_animal_talks(speaker: Box<dyn MakeNoise>) {
    println!("Welcome to the talk show");
    speaker.talk();
}
```
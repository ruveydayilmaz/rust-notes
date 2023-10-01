## Modules and Visibility

```RUST
fn main() {
    // Modules
    // Modules in Rust serve as a sort of namespace. 
    // They group related definitions together, and that could be anything - 
    // functions, structs, traits, enums, or even other modules.

    // Module syntax:
    // mod my_module {
    //      ...
    // }

    // Nested Module syntax:
    // mod my_module {
    //     mod my_nested_module {
    //         // ...
    //     }
    // }

    sports::football();    // This works!
    sports::basketball();  // This doesn't. basketball() is private to the sports module.

    my_module::public_function();  // Works fine
    my_module::private_function(); // Error! This function is private.
}

mod sports {
    pub fn football() { // pub: public
        println!("Football is fun!");
    }

    fn basketball() {
        println!("Basketball is intense!");
    }
}

mod my_module {
    pub fn public_function() {
        println!("I'm visible to the world!");
    }

    fn private_function() {
        println!("I'm shy...");
    }
}

mod my_module {
    pub struct Person {
        pub name: String,
        age: u8,
    }
}

```
## Enums

```RUST
fn main() {
    // Enums allows you to represent multiple related values within a single data type

    let _current_weather = Weather::Sunny; // Accessing a value from an Enum

    // Creating a Message enum variant with a String payload and processing it
    let msg = Message::Write(String::from("Hello Rust!"));
    process_message(msg); // Text message: Hello Rust!

    // Creating an Animal enum variant with a String payload and checking if it's a Cat
    let my_pet = Animal::Cat("Oreo".to_string());

    if let Animal::Cat(name) = my_pet {
        println!("My cat's name is {}", name);
    } else {
        println!("My pet is not a cat");
    }
    // result: My cat's name is Oreo

    // Creating a Message enum variant with a String payload and calling its associated method
    let msg = Message::Write(String::from("Oreo is sleeping"));
    msg.call(); // Write: Oreo is sleeping
}

enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // Implementing a method for the Message enu
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to R: {}, G: {}, B: {}", r, g, b);
            }
        }
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data");
        }
        Message::Move { x, y } => {
            println!("Move coordiantes x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
        }
    }
}

```
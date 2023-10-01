## Creating and using custom error types

```RUST
use std::fmt;

fn main() {
    let value = might_return_error()?;

    if let Err(e) = might_return_error() {
        println!("Error: {}", e);
    }
}

// basic custom error
fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("You can't divide by zero, silly!")
    } else {
        Ok(numerator / denominator)
    }
}

enum RocketError {
    OutOfFuel,
    NavigationSystemFailure,
    AlienInvasion,
}

fn handle_error(error: RocketError) {
    match error {
        RocketError::OutOfFuel => {
            println!("Houston, we have a problem. We're out of fuel!");
            // Trigger fuel replenishment procedures
        },
        RocketError::NavigationSystemFailure => {
            println!("Houston, we're lost. I knew we should've taken that left turn at Albuquerque!");
            // Trigger navigation system diagnostics
        },
        RocketError::AlienInvasion => {
            println!("Houston, we're not alone. There are aliens. This is not a drill!");
            // Trigger intergalactic diplomacy protocols
        },
    }
}

enum MyCustomError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Other(String),
}

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f, "I/O error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse error: {}", err),
            MyCustomError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

impl std::error::Error for MyCustomError {}

pub struct BookError {
    pub message: String,
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BookError {}

fn borrow_book(book_id: u32, library: &mut Library) -> Result<(), BookError> {
    if library.is_book_borrowed(book_id) {
        Err(BookError {
            message: String::from("Sorry, this book is already borrowed!"),
        })
    } else {
        library.borrow_book(book_id);
        Ok(())
    }
}

match might_return_error() {
    Ok(value) => println!("Got value: {}", value),
    Err(e) => match e {
        MyError::Type1 => println!("Error of Type 1 occurred"),
        MyError::Type2 => println!("Error of Type 2 occurred"),
    },
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Type1 => write!(f, "Type 1 error happened! Oh no!"),
            MyError::Type2 => write!(f, "Type 2 error happened! Not again!"),
        }
    }
}
```
## Option and Result

```RUST
fn main() {

    // Option
    // Option is an enum that represents the possibility of a value being present or absent.
    // It has two variants:
    // 1. Some(T): Represents a value of type T
    // 2. None: Represents the absence of a value

    // Since Rust doesn't have `null` type, Options are used to check the values.
    // The reason is to be secure.

    // Option usage
    enum Option<T> {
        Some(T),
        None,
    }

    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is {}", number, value),
        None => println!("The square root of {} is not a real number", number),
    } // Output: The square root of -4 is not a real number

    // Result usage
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);
    
    match division_result {
        Ok(value) => println!("{} divided by {} is {}", a, b, value),
        Err(error_message) => println!("Error {}", error_message),
    } // Output: Error Division by zero is not allowed

    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_triangle_area(base, height);

    match area_result {
        Ok(area) => println!("The area of the triangle is {}", area),
        Err(error_message) => println!("Error {}", error_message),
    } // Output: The area of the triangle is 12
    
}

// Function to find the square root of a number and return it as an Option
fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

// Function to perform division and return the result as a Result or an error message
fn divide(a: f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}

// Function to retrieve data from a 'database' based on a key
fn get_from_database(key: &str) -> Option<f64> {
    let database: [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(6.0))];

    for (k, v) in database {
        if k == key {
            return v;
        }
    }
    None
}

// Function to calculate the area of a triangle using base and height, returning a Result or an error message
fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base, height) {
        (Some(b), Some(h)) => {
            if b<= 0.0 || h <= 0.0 {
                Err("Both base and height must be positive numbers".to_string())
            } else {
                Ok(0.5 * b * h)
            }
        }
        (None, _) => Err("The base is missing".to_string()),
        (_, None) => Err("The height is missing".to_string()),
        // An underscore _ is used as a placeholder for a variable that you don't intend to use.
    }
}
```
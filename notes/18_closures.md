## Closures

```RUST
fn main() {
    // Closures
    // Closures are functions without names that can capture the enclosing environment. 
    // They are also known as anonymous functions or lambdas.

    // define a closure to print a text
    let print_text = || println!("Defining Closure");

    // call the closure
    print_text();

    // define a closure that takes an integer and returns a boolean
    let is_even = |x: i32| -> bool { x % 2 == 0 };

    // call the closure with different values
    println!("Is 2 even? {}", is_even(2)); // true
    println!("Is 3 even? {}", is_even(3)); // false

    // FnOnce: This trait represents closures that can be called exactly once. 
    // They may move (consume) values from their environment.
    
    // FnMut: This trait is for closures that can be called multiple times and can mutate values from their environment.
    
    // Fn: This trait is for closures that can be called multiple times without mutating their environment.

    // define a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];

    // filter out only the even numbers using a closure
    let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    // print the filtered vector
    println!("The even numbers are: {:?}", evens); // [2, 4]

    // define a closure that prints some text after receiving data
    let print_data = |data: &str| {
        println!("Received data: {}", data);
    };

    // call the download function with a url and a closure as arguments
    download_data("https://www.rust-lang.org", print_data);

    // Downloading data from https://www.rust-lang.org...
    // Received data: Some data from https://www.rust-lang.org
}

// define a function that downloads some data from a website
fn download_data(url: &str, callback: impl FnOnce(&str)) {
    // simulate downloading data by printing some text
    println!("Downloading data from {}...", url);

    // simulate some delay by sleeping for one second
    std::thread::sleep(std::time::Duration::from_secs(1));

    // simulate some data by creating a string
    let data = format!("Some data from {}", url);

    // execute the callback with the data as an argument
    callback(&data);
}
```
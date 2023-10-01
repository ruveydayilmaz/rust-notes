## Iterators with Loops

```RUST
use std::collections::HashMap;

fn main() {
    // for loop
    // general syntax:
    // for variable in iterator {
        // Code to execute for each element
    // }

    let numbers = vec![1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }

    for i in 0..5 { // 0..5 creates an iterator that yields the numbers from 0 to 4
        println!("{}", i);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);

    for (key, value) in scores.iter() { // Iterating over a HashMap
        println!("{}: {}", key, value);
    }

    // Iterator Methods
    // 1) Map: The map method is used to apply a function to each item of an 
    // iterator and transform it into a new iterator with the modified items.
    let doubled: Vec<i32> = numbers.iter().map(|x| x*2).collect(); // similar to map((x) => x * 2) in Javascript

    // 2) Filter: The filter method is used to create a new iterator 
    // containing only the elements that satisfy a certain condition.
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

    // 3) Fold: The fold method is used to accumulate the elements of an iterator into a single value.
    // It takes two arguments: 
        // - an initial value
        // - a closure that defines how to combine the accumulator with each element.
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

    // Chaining Iterator Methods
    let chained: Vec<i32> = nukbers.into_iter().filter(|n| n % 2 == 0).map(|n| n * 2).collect();
    let squared_numbers: HashMap<_, _> = numbers.iter().map(|n| (n, n * 2)).collect();
}


```
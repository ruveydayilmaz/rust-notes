## Error Handling

```RUST
use std::fs;

fn main() {

}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn read_number_from_file() -> Result<i32, std::io::Error> {
    let contents = fs::read_to_string("my_number.txt")?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn get_ingredient(name: &str) -> Result<Option<String>, &'static str> {
    let pantry: HashMap<String, Option<String>> = // ... your pantry

    match pantry.get(name) {
        Some(ingredient) => {
            match ingredient {
                Some(details) => Ok(Some(details.to_string())),
                None => Ok(None),
            }
        },
        None => Err("Ingredient not found!"),
    }
}

let eggs = get_ingredient("eggs");

match eggs {
    Ok(Some(details)) => println!("We have eggs: {}", details),
    Ok(None) => println!("No")
}
```
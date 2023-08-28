## Functions and Control Flow

```RUST
fn main() {
    let sum = add(3, 5);
    println!("The sum is {}", sum);

    let day_of_the_week = "Sunday";

    // if statement
    if day_of_the_week == "Sunday" {
        println!("Race day!");
    } else if day_of_the_week == "Saturday" {
        println!("Qualifying today!")
    } else {
        println!("Waiting for the race day😭");
    }

    // while loop
    let mut counter = 0;
    while counter < 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }

    // loop
    counter = 0;
    loop {
        println!("Counter value is {}", counter);
        counter +=1;

        if counter == 6 {
            break;
        }
    }

    // match (similar to switch case)
    let num = 5;
    match num {
        1 => {
            println!("The number is 1");
            println!("This is the first match arm!");
        }
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        _ => println!("The number is something else"),
    }

    let result = match num2 {
        1 => "The number is 1",
        2 => "The number is 2",
        3 => "The number is 3",
        _ => "The number is something else",
    };

    println!("The result is {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}

fn no_param() -> i32 {
    println!("This just works");
    1 // return 1;
}
```

use std::io;

fn main() -> io::Result<()> {
    let mut first_input = String::new();
    let mut second_input = String::new();
    let mut operation_input = String::new();

    let stdin = io::stdin();

    println!("Enter the first number: ");
    stdin.read_line(&mut first_input)?;

    println!("Enter the operation (+, -, *, /): ");
    stdin.read_line(&mut operation_input)?;

    println!("Enter the second number: ");
    stdin.read_line(&mut second_input)?;

    let first_input: f64 = first_input.trim().parse().unwrap();
    let second_input: f64 = second_input.trim().parse().unwrap();

    let operation = match operation_input.trim() {
        "+" => Operation::Add(first_input, second_input),
        "-" => Operation::Subtract(first_input, second_input),
        "*" => Operation::Multiply(first_input, second_input),
        "/" => Operation::Divide(first_input, second_input),
        _ => {
            println!("Invalid operation");
            return Ok(());
        }
    };

    let calculate_result = calculate(operation);

    println!("Calculated result is {}", calculate_result);

    Ok(())
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

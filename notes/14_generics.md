## Generics

```RUST
fn main() {
    // Generics
    // Generics help us reduce code duplication and make our code more reusable.
    // Instead of having multiple functions that are almost the same, 
    // we can write one function that can work with any type we throw at it.

    let int_container = Container { value: 42 };
    let str_container = Container { value: "Simon" };

    println!("{}", int_container.value); // Output: 42
    println!("{}", str_container.value); // Output: Simon

    let success_result: MagicalResult<i32, String> = MagicalResult::Success(42);
    let failure_result: MagicalResult<i32, String> = MagicalResult::Failure("it failed".to_string());

    match success_result {
        MagicalResult::Success((value) => println!("we got success {}", value)),
        MagicalResult::Failure((value) => println!("we got failure {}", value)),
    }

    let mut honda = ElectricCar { battery_level: 42 };
    let mut ford = GasCar { gas_level: 20 };

    honda.refuel(fuel: 58); // Output: Battery charged to 100%
    for.refuel(fuel: 0.2); // Gas tank filled to 40%
}

fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let temp *x;
    *x = *y;
    *y = temp;
}

trait Summary {
    fn summarize(&self) -> String;
}

fn print_summary<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

// By using the 'where' clause, we can specify more complex trait bounds in an organized manner.
fn print_double_summary<T, U>(item1: T, item2: U)
where 
    T: Summary,
    U: Summary + Clone,
{
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());

    let cloned_item = item2.clone();
    println!("{}", cloned_item.summarize());
}

struct Container<T> {
    value T;
}

enum MagicalResult<T, E> {
    Success(T),
    Failure(E),
}

trait Vehicle {
    type Fuel;

    fn refuel(&mut self, fuel: Self::Fuel);
}

struct ElectricCar {
    battery_level: u32,
}

struct GasCar {
    gas_level: u32,
}

imp Vehicle for ElectricCar {
    type Fuel = u32;

    fn refuel(&mut self, charge: Self::Fuel) {
        self.battery_level += charge;
        println!("Battery charged to {}%", self.battery_level);
    }
}

imp Vehicle for GasCar {
    type Fuel = u32;

    fn refuel(&mut self, gas: Self::Fuel) {
        self.gas_level += (gas * 100.0) as u32;
        println!("Gas tank filled to {}%", self.gas_level);
    }
}
```
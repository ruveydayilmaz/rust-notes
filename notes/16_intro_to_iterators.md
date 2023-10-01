## Introduction to Iterators and it's types

```RUST
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut iter = numbers.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);

    // In this example, we have a vector of numbers, and we create an iterator over it using the iter() method. 
    // We then call the next() method on the iterator to get each value in the sequence, one by one. 
    // When there are no more values left, the next() method returns None.

    let mut fibonacci = Fibonacci {current: 0, next: 1};

    for _ in 0..10 {
        println!("{}", fibonacci.next().unwrap());
    }
    // Output:
    // 0
    // 1
    // 1
    // 2
    // 3
    // 5
    // 8

    // Iterator Types
    // 1) iter: This iterator type is used when you want to iterate over a collection by borrowing its elements.
    let vec = vec![1, 2, 3, 4, 5];
    for item in vec.iter() {
        println!("{}", item);
    }

    // 2) iter_mut: This iterator type is used when you want to iterate over a mutable collection by borrowing its elements mutably.
    let mut vec2 = vec![1, 2, 3, 4, 5];
    for item in vec2.iter_mut() {
        *item *= 2;
    }

    // 3) into_iter: This iterator type is used when you want to iterate over a collection by consuming it.
    let vec = vec![1, 2, 3, 4, 5];
    for item in vec.into_iter() {
        println!("{}", item);
    }

}

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Fibonacci {
    current: u32,
    next: u32,
}

imp Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        selft.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}
```
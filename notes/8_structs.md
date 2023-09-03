## Structs

```RUST
fn main() {
    // Structs are custom data types that help you group related data together. 

    // Creating a Book struct instance
    let book = Book {
        title: String::from("The Way of Zen"),
        author: String::from("Alan Watts"),
        publication_year: 1957,
    };

    println!(
        "The book {} is written by {} in {}", 
        book.title, book.author, book.publication_year
    ); // The book The Way of Zen is written by Alan Watts in 1957

    // Modifying a mutable Book struct instance
    let mut book = Book {
        title: String::from("The Way of Zen"),
        author: String::from("Alan Watts"),
        publication_year: 1957,
    };

    book.publication_year = 1989; // Updating the publication_year

    println!(
        "The book {} is written by {} in {}", 
        book.title, book.author, book.publication_year
    ); // The book The Way of Zen is written by Alan Watts in 1989

    let book_data = get_book_data(book);

    for data in book_data {
        println!("{data}");
        // The Way of Zen
        // Alan Watts
        // 1989
    }

    // Creating a Book struct using a function
    let my_book = create_book(
        "The Path of Zen".to_string(), 
        "Simon".to_string(), 
        2023
    );

    // Printing the struct using debug notation
    println!("My book is {:?}", my_book);
    // My book is Book { title: "The Path of Zen", author: "Simon", publication_year: 2023 }

    // Tuple Structs
    // A tuple struct is similar to a regular struct, but its fields do not have names. 
    // Instead, they are accessed by their position, just like the elements of a tuple.

    let tuple_book = TupleBook(
        "Some book".to_string(), 
        "Simon".to_string(), 
        2023
    );

    // Accessing tuple struct fields
    let _title = tuple_book.0;
    let _author = tuple_book.1;
    let _publication_year = tuple_book.2;

    // Unit Struct
    // Unit structs have no fields and are used to create distinct types.

    let _unit_book = UnitBook;

    let my_rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let area = my_rectangle.area(); // Calling the `area` method associated with the struct
    println!("The area of the rectangle is: {}", area); // The area of the rectangle is: 50

}

#[derive(Debug)] // Enables debugging for the Book struct
struct Book { // Normal Struct
    title: String,
    author: String,
    publication_year: u32,
}

struct TupleBook(String, String, u32); // Tuple Struct

struct UnitBook; // Unit Struct

// Defining methods in Rust
// 1. Define the struct
// 2. Define the methods associated with the struct in a seperate place
struct Rectangle { // 1
    width: f64,
    height: f64,
}

impl Rectangle { // 2
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    // Converting publication_year to a string to ensure it matches the array's data type,
    // as arrays can only hold values of the same data type

    data
}

fn create_book(title: String, author: String, publication_year: u32) -> Book {
    let book = Book {
        // Rather than writing `title: title`, we can simple write `title`
        title, // Same as `title: title`
        author,
        publication_year,
    };

    book
}
```
## Lifetimes

```RUST
fn main() {
    // A lifetime is a construct of the compiler uses to ensure all borrows are valid. 
    // Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. 
    // While lifetimes and scopes are often referred to together, they are not the same.

    let name = String::from("Elon Musk");
    let person = Person { name: &name };
    println!("The person's name is: {}", person.name);

    // Static Lifetimes
    let s: &'static str = "I'm immortal!";
    static HELLO_WORLD: &str = "Hello, world!";

    let parent: &'static str = "I'm here forever!";
    let kid: &'static str = {
        let short_lifetime_str = String::from("Just passing through!");
        &short_lifetime_str // Error here! This won't work
    };

    let name = String::from("Great-Grandpa");
    let person: Ancestor<'static> = Ancestor(&name); // Error! `name` doesn't live long enough

    // Best Practices
    // 1) Don't Fight the Borrow Checker

    // 2) Embrace Lifetime Elision: Explicit lifetime annotations are important but 
    // let's not overdo them.

    // 3) Keep it Simple: Lifetimes can get complicated.
    // Try to keep your code as simple and straightforward as possible.
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct Sentence<'a> {
    content: &'a str,
}

imp<'a>Sentence<'a> {
    fn yell(&self) -> &str {
        return "Do not code until 3am..";
    }
}

fn no_no_function<'a>(x: &'a str, y: &'a str) -> &'a str { // Compile Error
    let some_string = String::from(x);
    &some_string 
    // The result variable is a temporary string that's discarded when the function ends.
    // Rust complains if we try to return a reference to it because it's about to vanish. 
    // To resolve this, we must ensure our output has a lasting reference to return.
}

struct Person<'a> {
    name: &'a str,
}

struct LongLived<'a>(&'a str);

struct ShortLived<'a> {
    name: &'a str,
    long: LongLived<'a>,
}

struct Ancestor<'a>(&'a str);

fn problematic_function() -> &str {
    let string_inside = String::from("Hello, Rust!");
    &string_inside[..]
    // The function tries to return a reference to string_inside, 
    // but string_inside goes out of scope when the function ends.
}

fn better_function() -> String {
    let string_inside = String::from("Hello, Rust!");
    string_inside
}
```
fn main() {
    let string1 = String::from("first string, ");
    let string2 = String::from("second string");

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}", concatenated_string);
}

fn concatenate_strings (x: &str, y: &str) -> String {
    let mut result = String::new();
    result.push_str(x);
    result.push_str(y);

    result
}
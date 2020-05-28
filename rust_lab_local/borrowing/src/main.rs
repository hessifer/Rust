fn main() {
    let mut s = String::from("book");

    println!("The value of s is now {}.", pluralize(&mut s));
}

// Add appropriate parameters, return values, and implementation to this function
fn pluralize(s: &mut String) -> &str {
    // s + "s"
     s.push_str("s");
     s
}
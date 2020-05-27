fn main() {
    let s = String::from("book");
    
    // Add code here that calls the pluralize function
    
    println!(
        "I have one {}, you have two {}",
        s,
        // ugly, borrowing is what we need, but not part of
        // this exercise
        pluralize(s.clone()), // slow, non-performant (deep copy)
    );
}

// Add appropriate parameters, return values, and implementation to this function
fn pluralize(mut s: String) -> String {
    // s + "s"
     s.push_str("s");
     s
}
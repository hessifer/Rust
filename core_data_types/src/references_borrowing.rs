// Rust allows us to use a value but not take ownership of it via 'references' (&)
// When a function takes a reference as a parameter this is called 'borrowing'
// Just like variables, references are 'immutable' by default

pub fn calculate_length_of_string(s: &String) -> usize { // function takes a reference to a String
    s.len() // return the length of s (no semi-colon as we are returning the result of this expression
} // s goes out of scope, but since it does not have ownership nothing happens


pub fn modify_greeting(greeting: &mut String) { // take a mutable reference and modify it
    greeting.push_str(", in Las Tablas!");
}
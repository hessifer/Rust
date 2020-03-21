fn main() {
    let mut message = String::from("Hello World!");
    let add_text = String::from(" Dude");
    println!("The length of {} is {}.", message, calculate_length(&message));
    append_text(&mut message, &add_text);
    println!("The new message is: {}", message);
}

// pass by reference, do not take ownership
// below we are borrowing the variable passed as an argument
// references are immutable by default
fn calculate_length(text: &String) -> usize {
    text.len()
}

// mutable refrence
fn append_text(text: &mut String, text_to_add: &String) {
    text.push_str(text_to_add);
}
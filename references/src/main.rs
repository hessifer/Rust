fn main() {
    let s = String::from("Charles");
    let len = calculate_length(&s);
    let mut msg = String::from("Hello"); // mutable String

    println!("Old Message: {}", msg);
    change_reference(&mut msg); // pass mutable reference, only one mutable reference to same data 
    println!("New Message: {}", msg);
    println!("The length of {} is {}.", s, len);

    println!("{}", first_word(&msg));
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change_reference(msg: &mut String) { // function that takes mutable reference param
    msg.push_str(" World!");
}

fn first_word(w: &str) -> &str {  // reference to str slice as parameter, return reference to str literal
    // change str into byte array
    let bytes = w.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // if we reach a ' ' in our ref to String, grab current index
            return &w[..i]; // return a reference to our slice
        }
    }
    &w[..]
}

fn print_vowels(txt: &str) {
    // function to display the vowels in a word

}

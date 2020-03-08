fn main() {
    let full_name = "Charles Hessifer";

    println!("First Name: {}", first_word(full_name)); // string literal is a slice (immutable)
    println!("Last Name: {}", second_word(full_name)); // string literal is a slice (immutable)
}

fn first_word(text: &str) -> &str {
    let text_bytes = text.as_bytes();

    // iterate text and grab slice up to space
    for (i, &item) in text_bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }
    &text[..]
}

fn second_word(text: &str) -> &str {
    let text_bytes = text.as_bytes();

    for (i, &item) in text_bytes.iter().enumerate() {
        if item == b' ' {
            return &text[i+1..];
        }
    }
    &text
}

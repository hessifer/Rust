fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut words = Vec::new();
    let mut offset = 0;

    for (i, &item) in bytes.iter().enumerate() {
        // capture words but ignore blank spaces
        if item == b' ' {
            if offset == 0 {
                words.push(&s[offset..i]);
                offset = i;
            } else {
                words.push(&s[offset + 1..i]);
                offset = i;
            }
        }
    }

    // capture last word
    if offset > 0 {
        words.push(&s[offset..bytes.len()].trim());
    }

    println!("Number of words: {}", words.len());
    println!("Words: {:?}", words);
    words[0]
}

fn main() {
    let s = String::from("All Work & No Play");
    let word = first_word(&s);
    // s.clear(); // empties string

    println!("First Word: {}", word);
}

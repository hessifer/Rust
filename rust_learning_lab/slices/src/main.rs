fn get_words(s: &String) -> Vec<&str> {
    let bytes = s.as_bytes();
    let mut words = Vec::new();
    let mut pos = 0;

    for (i, &item) in bytes.iter().enumerate() {
        // capture words but ignore blank spaces
        if item == b' ' {
            if pos == 0 {
                words.push(&s[pos..i]);
                pos = i;
            } else {
                words.push(&s[pos + 1..i]);
                pos = i;
            }
        }
    }

    // capture last word
    if pos > 0 {
        if pos != &s.len() - 1 {
            // words.push(&s[pos..bytes.len()].trim());
            words.push(&s[pos + 1..bytes.len()]);
        }
    }

    println!("Number of words: {}", words.len());
    println!("Words: {:?}", words);
    words
}

fn main() {
    let s = String::from("All Work & No Play ");
    let words = get_words(&s);
    // s.clear(); // empties string

    println!("First Word: {}", words[0]);
    println!("Last Word: {}", words[words.len() - 1]);
}

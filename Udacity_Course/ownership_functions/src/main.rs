fn main() {
    let my_string = String::from("Rust Rocks!");
    let occurence_count = count_occurences(my_string.clone(), 'R');

    println!("The number of times 'R' appears in \"{my_string}\" = {occurence_count}");
}

fn count_occurences(text: String, letter: char) -> u32 {
    let mut res = 0;
    for ch in text.chars() {
        if ch == letter {
            res += 1;
        }
    }
    res
}

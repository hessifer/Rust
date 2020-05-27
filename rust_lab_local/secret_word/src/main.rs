use std::io;

fn main() {
    let mut word = String::new();

    while word.trim() != "rust" {
        println!("What's the secret word?");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }

    println!("You know the secret word! Please proceed!");

    let number = 3;

    match number {
        1 => println!("One is the loneliest number."),
        2 => println!("Two for me and you."),
        3 => println!("Three's a crowd."),
        _ => println!("Found the bit bucket."),
    }
}

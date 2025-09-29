use std::io;
use std::io::{Write}; 

fn main() {
    // ask for a number
    print!("Please enter a number: ");

    // flush stdout to prompt immediately
    io::stdout().flush().expect("Failed to flush stdout");

    // read input line
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {e}");
        return; // Exit on IO Error
    }

    let trimmed_input = input.trim();

    match trimmed_input.parse::<i32>() {
        Ok(number) => {
            println!("The number is {number}");       
            if number % 2 == 0 {
                println!("{number} is even.");
            } else {
                println!("{number} is odd.");
            }
        }
        Err(e) => {
            eprintln!("Error: Could not parse {trimmed_input} as an integer. Details {e}");
        }
    }
}

use std::io;
use std::io::{Write};

fn some_numbers(end: i32, inclusive: bool) -> i32 {
    // optimize for sum of an arithmetic series k * (k + 1) / 2
    if inclusive {
        end * (end + 1) / 2
    } else {
        (end - 1) * end / 2
    }
}

fn main() {
    let mut input = String::new();
    print!("Please enter a number and I will tell you the sum of all numbers from 1 to your number (inclusive): ");
    io::stdout().flush().expect("Error: could not flush stdout.");

    
    // if let since we only care if it errors
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {e}");
    }
    let trimmed_output = input.trim();
    match trimmed_output.parse::<i32>() {
        Ok(end) => {
            println!("The sum of all numbers from 1 to {end} is {}", some_numbers(end, true));
        },
        Err(e) => eprintln!("Error: could not parse {input} as an integer. Details: {e}"),   
    }
}

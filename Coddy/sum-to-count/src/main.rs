use std::io;
use std::io::Write;

fn main() {
    io::stdout().flush().expect("Failed to flush stdout");  // prompt immediately
    
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        return;
    }
    
    let input_trimmed = input.trim();
    
    match input_trimmed.parse::<i32>() {
        Ok(count) => {
            let mut sum: i32 = 0;
            for _i in 1..=count {
                let mut input: String = String::new();
                io::stdout().flush().expect("Failed to flush stdout");  // prompt immediately
                if let Err(e) = io::stdin().read_line(&mut input) {
                    eprintln!("Error reading input: {}", e);
                }
                let input_trimmed = input.trim();
                match input_trimmed.parse::<i32>() {
                    Ok(num) => {
                        sum += num;
                    },
                    Err(e) => {
                        eprintln!("Error parsing input: {}", e);
                        return;
                    }
                };
            }
            println!("{sum}");
        },
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
        }
    };
}
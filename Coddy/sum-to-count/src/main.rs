use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut buffer) {
        eprintln!("Error reading input: {}", e);
        return;
    }

    let mut tokens = buffer.split_whitespace();

    let count: usize = match tokens.next().map(|t| t.parse::<usize>()) {
        Some(Ok(c)) => c,
        Some(Err(e)) => {
            eprintln!("Error parsing input: {}", e);
            return;
        }
        None => {
            eprintln!("Error parsing input: missing count");
            return;
        }
    };

    let mut sum: i32 = 0;
    for _ in 0..count {
        let value = match tokens.next().map(|t| t.parse::<i32>()) {
            Some(Ok(n)) => n,
            Some(Err(e)) => {
                eprintln!("Error parsing input: {}", e);
                return;
            }
            None => {
                eprintln!("Error parsing input: not enough numbers");
                return;
            }
        };
        sum += value;
    }

    println!("{sum}");
}

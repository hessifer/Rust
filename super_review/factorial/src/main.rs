use std::io::{self, Write};

fn factorial(number: usize) -> usize {
    let mut num = number;
    let mut result = 1;
    
    while num > 1 {
        result = result * num;
        num = num - 1;
    }
    result
}

fn main() {
    let mut answer = String::from("y");
    let mut number = String::new();
    
    while answer == "y" {
        print!("Enter a number and I will calculate the factorial: ");
        io::stdout().flush().unwrap(); // flush if line buffered
        io::stdin().read_line(&mut number).unwrap();
        
        match number.trim_end().parse::<usize>() {
            Ok(num) => println!("The factorial of {} is {}.", num, factorial(num)),
            Err(..) => println!("You did not enter a valid number: {}", number),
        };
        
        
        print!("Would you like to continue? [y/n]: ");
        io::stdout().flush().unwrap(); // flush if line buffered
        
        match answer.trim_end().parse::String() {
            Ok(a) => answer = a,
            Err(error) => println!("Error: {}", error),
        }
    }
}
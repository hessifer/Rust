use std::io;

/*
    Program: quiz
    Author:  unknown
    Date:    1/19/19
    Purpose: Ask a user a question and give them three chances to
             get the correct answer.
*/

fn main() {
    let mut answer = String::new();

    println!("What is the capital of Virginia?");
    for n in 1..4 {
        answer.clear();
        io::stdin().read_line(&mut answer).expect("ERROR: could not parse answer.");
        answer = answer.trim().to_string();

        if answer == "richmond" || answer == "Richmond" {
            println!("You guessed correctly!");
            break;
        } else {
            if n < 3 {
                println!("Try again!");
            } else {
                println!("Better luck next time.");
            }
        }
    }
}

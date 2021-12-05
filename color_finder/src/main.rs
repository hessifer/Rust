use std::io;
use std::io::Write;

fn main() {
    let colors = ["sword", "ruby", "shield", "wand"];
    let mut guess = String::new();

    print!("Guess an item and see if it is in the treasure chest: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess).unwrap();
    let trim_guess = guess.trim();

    let result = match trim_guess {
        "sword" => format!("You have found a bronze {}.", trim_guess),
        "ruby" => format!("You have found a large glowing {}.", trim_guess),
        "shield" => format!("You have found a tower {}.", trim_guess),
        "wand" => format!("You have found a magic {}.", trim_guess),
        _ => format!("{} was not in the treasure chest.", trim_guess),
    };

    println!("{}", result);
    println!("\nThe items in the teasure chest were:");
    for c in colors {
        print!("{} ", c);
    }
    print!("\n");
}

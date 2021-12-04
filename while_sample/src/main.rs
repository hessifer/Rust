use std::io;
use std::io::Write;

fn main() {
    let mut even_numbers_entered = vec![];

    while even_numbers_entered.len() < 5 {
        // get number from user
        let mut entry = String::new();
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut entry).unwrap();

        let ientry = match entry.trim().parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1,
        };

        // if number entered is even add to vector
        if ientry % 2 != 0 {
            continue;
        }
        even_numbers_entered.push(ientry);
    }

    println!("\nYou entered the following even numbers:");
    for num in even_numbers_entered {
        println!("{}", num);
    }
}

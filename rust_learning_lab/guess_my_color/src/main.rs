use std::io::stdin;

fn main() {
    let mut input = String::new();
    let magic_color = String::from("red");

    println!("Please try and guess the magic color: ");
    match stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim_end().to_lowercase() == magic_color {
                println!("You guessed the magic color: {}", magic_color);
            }
        },
        Err(e) => println!("Looks like we had an error - {}", e)
    }
}

use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name: String = what_is_your_name();
    let visitor_list: [&str; 3] = ["bob", "bert", "charles"];
    let mut allow_them_in: bool = false;
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
    // println!("Hello, {}", what_is_your_name().to_uppercase());
}

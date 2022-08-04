use std::io::stdin;

struct User {
    name: String,
    greeting: String,
}

impl User {
    // constructor
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greeting(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut name: String = String::new();
    stdin()
        .read_line(&mut name)
        .expect("ERROR: failed to read line.");
    name.trim().to_lowercase()
}

fn main() {
    // main code here
    println!("Hello, what is your name?");
    let name = what_is_your_name();
    println!("Hello {}, let's look at our user list.", name.to_uppercase());
    let user_list = [
        User::new("charles", "Hello World!"),
    ];

    for user in &user_list {
        print!("{} - ", user.name);
        user.greeting();
        println!();
    }
}

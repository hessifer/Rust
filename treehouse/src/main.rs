use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    // constructor
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => {
                println!("Welcome to the treehouse, {}", self.name.to_uppercase())
            }
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name.to_uppercase());
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name.to_uppercase());
                }
            }
            VisitorAction::Probation => {
                println!("{} is now a probationary member.", self.name.to_uppercase())
            }
            VisitorAction::Refuse => println!("Do not allow {} in.", self.name.to_uppercase()),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
<<<<<<< HEAD
    println!("Hello, what's your name?");
    let name: String = what_is_your_name();
    let visitor_list: [Visitor; 3] = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi, Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    // let mut allow_them_in: bool = false;
    /*
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
=======
    // Loop until user enters 'q'
    loop {
        println!("Hello, what's your name? [Enter q to quit]: ");
        let name: String = what_is_your_name();

        // List of visitors (Uses Visitor constructor to populate)
        let mut visitor_list = vec![
            Visitor::new("Bert", VisitorAction::Accept, 45),
            Visitor::new(
                "Steve",
                VisitorAction::AcceptWithNote {
                    note: String::from("Lactose-free milk is in the fridge."),
                },
                15,
            ),
            Visitor::new("Fred", VisitorAction::Refuse, 30),
            Visitor::new("Charles", VisitorAction::Probation, 47),
        ];

        // Greet known visitors or add new ones, provided entry is not q
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name == "q" {
                    break; // exit main loop
                } else {
                    println!("{} is not on the visitor list!", name.to_uppercase());
                    visitor_list.push(Visitor::new(
                        &name,
                        VisitorAction::AcceptWithNote {
                            note: "New friend".to_string(),
                        },
                        0,
                    ));
                }
            }
>>>>>>> 4ed047fc720be6eaba28dc15537a43ba9619af9d
        }

        // Pretty Print visitor_list Vector
        println!("\n\nContents of Visitor List:");
        println!("{:#?}", visitor_list);
    }
<<<<<<< HEAD
    */

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
    
    /*
    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
    */
    // println!("Hello, {}", what_is_your_name().to_uppercase());
=======
>>>>>>> 4ed047fc720be6eaba28dc15537a43ba9619af9d
}

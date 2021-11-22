#[derive(Debug)]
pub struct Person {
    // data
    name: String, // pointer
    age: i32,
    children: i32,
    fav_color: Color,
}

impl Person {
    // implementation
    pub fn print(self) -> String {
        format!("name = {}, age = {} has {} children", self.name, self.age, self.children)
    }
}

#[derive(Debug)]
pub enum Color {
    Red(String), // enums are powerful and can include other types, largest type in enum is what is used to allocate memory
    Green,
    Blue,
}

fn main() {
    let p = Person {
        name: "matt".to_string(),
        age: 35,
        children: 4,
        fav_color: Color::Blue,
    };

    let c = Color::Red("hello, I am Red!".to_string());

    match c {
        Color::Red(s) => println!("{}", s), // if  you don't use the value in Red() use an _
        Color::Blue => println!("It's blue"),
        Color::Green => println!("It's green"),
    }

    println!("Hello, people from {:?}", p); // used in combo with #[derive(Debug]
    println!("Hello, people from {}", p.print());
}

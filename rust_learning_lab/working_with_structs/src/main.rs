struct User {
    name: String,
    age:  i32,
    height: i32,
    weight: i32,
    shoesize: i32,
}

impl User {
    fn simple_string(&self) -> String {
        format!("User: {}\nAge: {}\nHeight: {}\"\nWeight: {} lbs\nShoe Size: {}\n\n", self.name, self.age, self.height, self.weight, self.shoesize)
    }
}

fn main() {
    let u = User {
        name: "Charles".to_string(),
        age: 45,
        height: 68,
        weight: 175,
        shoesize: 9,
    };

    println!("{}", u.simple_string());
}

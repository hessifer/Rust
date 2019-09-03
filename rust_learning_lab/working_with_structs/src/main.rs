struct User {
    name: String,
    age: i32,
    height: i32,
    weight: i32,
    shoesize: i32,
}

impl User {
    fn simple_string(&self) -> String {
        format!(
            "User: {}\nAge: {}\nHeight: {}\"\nWeight: {} lbs\nShoe Size: {}\n\n",
            self.name, self.age, self.height, self.weight, self.shoesize
        )
    }

    fn grow(&mut self, h: i32) {
        self.height += h;
    }

    fn die(self) {
        println!("Dead {} ", self.simple_string());
    }
}

fn main() {
    let mut u = User {
        name: "Charles".to_string(),
        age: 45,
        height: 68,
        weight: 175,
        shoesize: 9,
    };

    println!("{}", u.simple_string());
    u.grow(3);
    println!("{}", u.simple_string());

    u.die(); // u is now consumed and can no longer be used
             // u.grow(10); // will error as 'u' has moved
}

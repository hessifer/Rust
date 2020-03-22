use std::io;

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

// Student
struct Student {
    first_name: String,
    last_name: String,
    student_id: usize,
    courses: Vec<String>,
}

impl Student {
    fn add_course(&mut self, course: &str) {
        self.courses.push(course.to_string());
    }
    
    fn list_courses(&self) {
        println!("{}", &self.courses.join(", "));
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


    // Student Example
    let mut charles = Student {
        first_name: "Charles".to_string(),
        last_name: "Hessifer".to_string(),
        student_id: 000001,
        courses: Vec::new(),
    };

    // Add course
    let mut input = String::new();
    println!("What course would you like to enroll in?");
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => charles.add_course(&input.trim_end()),
        Err(_) => println!("Unable to enroll in course: {}", input),
    }
    println!("Student ID: {}\nName: {} {}", charles.student_id, charles.first_name, charles.last_name);
    print!("Courses: ");
    charles.add_course("CS 101");
    charles.list_courses();
    println!();


    // Example of a Tuple Struct
    // Tuple structs have a name for the struct, 
    // but just the type for the fields. Good for
    // naming a tuple.
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: {:?}\tOrigin: {:?}", black, origin);
}

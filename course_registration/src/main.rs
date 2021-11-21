use std::io;
use std::io::Write; // flush()

fn main() {
    let mut fname: String = String::new();
    let mut lname: String = String::new();
    let mut email: String = String::new();
    let mut phone: String = String::new();

    print!("Please enter your first name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fname).unwrap();
    fname.truncate(fname.len() - 1);
    // dbg!(&fname);

    print!("Please enter your last name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lname).unwrap();
    lname.truncate(lname.len() - 1);

    print!("Please enter your email address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email).unwrap();
    email.truncate(email.len() - 1);

    print!("Please enter your phone number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut phone).unwrap();
    phone.truncate(phone.len() - 1);

    let student = Student::new(fname, lname, email, phone);

    println!("\n\nThanks for registering, your student information follows:");
    println!("Student Name: {} {}\nStudent ID: {}\nEmail: {}\nPhone: {}\n", student.first_name, student.last_name, student.id,student.email, student.phone);
}

struct Student {
    first_name: String,
    last_name: String,
    id: usize,
    email: String,
    phone: String,
}

impl Student {
    // constructor for new Student
    fn new(first_name: String, last_name: String, email: String, phone: String) -> Self {
        Self {
            first_name,
            last_name,
            id: Student::generate_student_id(),
            email,
            phone
        }
    }

    fn generate_student_id() -> usize {
        100
    }
}

/*
struct Course {
    course: String,
    id: usize,
}
 */
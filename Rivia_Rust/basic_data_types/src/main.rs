fn main() {
    // variable declarations
    let my_age: u8 = 27;
    let first_letter_of_name: char = 'c';
    let interest_rate: f64 = 3.96;
    let message: String = String::from("Hello!");
    let colors: [&str; 3] = ["reg", "green", "blue"];
    let data: (u8, f64, String) = (my_age, interest_rate, message);

    println!("Age: {}", my_age);
    println!("Char: {}", first_letter_of_name);
    println!("First Color: {}", colors[0]);
    println!("Age: {} - Interest: {} - Message: {}", data.0, data.1, data.2);

    // Destructure tuple example
    let (x, y, z) = data;
    println!("{}, {}, {}", x, y, z);
}

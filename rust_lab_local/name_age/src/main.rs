fn main() {
    let name = "Charles";
    let age = 27;

    display_name_age(name, age);
    println!("The sum of 2 + 2 is {}", sum_two_numbers(2, 2)); 
}

fn display_name_age(name: &str, age: u8) {
    println!("Name: {}\nAge: {}", name, age);
}

fn sum_two_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
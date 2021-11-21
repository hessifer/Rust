use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Enter your weight in pounds: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(pounds_to_kilograms(weight));
    println!("\nYour weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(w: f32) -> f32 {
    (w / 9.81) * 3.711
}

fn pounds_to_kilograms(pounds: f32) -> f32 {
    pounds / 2.205
}

fn main() {
    let my_numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in my_numbers.iter() {
        if i % 2 == 0 {
            println!("{} is even.", i);
        }
    }
}

use std::io;

fn main() {
    let mut start_number = String::new();
    let mut stop_number = String::new();

    println!("Please enter a start and stop number.");
    io::stdin().read_line(&mut start_number).expect("ERROR: could not parse start number.");
    io::stdin().read_line(&mut stop_number).expect("ERROR: could not parse stop number.");
    let start_number:usize = start_number.trim().parse().expect("ERROR: could not parse start number.");
    let stop_number:usize = stop_number.trim().parse().expect("ERROR: could not parse start number.");

    for num in start_number..stop_number {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}

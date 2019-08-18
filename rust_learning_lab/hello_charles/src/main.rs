fn main() {
    let mut my_number = 5;

    if my_number % 2 == 0 {
        println!("{} is an even number.", my_number);
    } else {
        println!("{} is an odd number.", my_number);
    }
    my_number += 1;

    println!("{}", my_number);
}

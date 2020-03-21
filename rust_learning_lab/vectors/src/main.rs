fn main() {
    let mut v = Vec::new(); // vector already included in prelude (no need to import)

    v.push('a');
    v.push('b');
    v.push('c');
    match v.pop() {
        Some(value) => println!("Last element in vector v is: {}.", value),
        None => println!("Unable to determine last element of vector v."),
    }
    
    // loop over vector and display its contents
    for item in v {
        print!("{}", item);
    }
}

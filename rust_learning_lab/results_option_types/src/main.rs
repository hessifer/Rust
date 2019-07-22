use std::collections::HashMap;
use std::env::args;

fn main() {
    let mut hm = HashMap::new();
    hm.insert(3, "Hello");
    hm.insert(5, "World");

    // let r = hm.get(&4).unwrap(); // will panic
    // let r = hm.get(&4).unwrap_or(&"NoString");
    let r = match hm.get(&3) { // r is an option (enum)
        Some(v) => v,
        _ => "NOTHING",
    };

    println!("{}", r);

    match "3t".parse::<f32>() { // could use .expect(msg) here to provide more info on panic
        Ok(v) => println!("OK = {}", v),
        Err(e) => println!("Error: {}", e),
    }

    match get_args(3) {
        Ok(v) => println!("OK = {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

// you can pass arguments to cargo after -- (cargo run -- arg1 arg2 arg3)
fn get_args(n: usize) -> Result<String, String> { // better to use a enum
    for (i, a) in args().enumerate() {
        if i == n {
            return Ok(a)
        }
    }
    Err("Not enough args".to_string())
}

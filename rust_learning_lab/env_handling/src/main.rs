use std::env;

fn main() {
    let vars = env::vars();

    // Print all environment variables
    for (key, val) in vars {
        println!("{}:{}", key, val);
    }
}

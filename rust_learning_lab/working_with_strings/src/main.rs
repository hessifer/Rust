fn main() {
    let last_name = String::from("Charles"); // string array owns the data, mutable
    
    let first_name = "bob";  // ptr to data, no ownership and immutable
    
    for i in first_name.chars() {
        print!("{} ", i.to_lowercase());
    }
    
    println!("\n");
    
    for c in last_name.chars() {
        print!("{} ", c.to_lowercase());
    }
    println!();
    println!("Size of Charles in bytes: {}", last_name.len());
    
    for (i, c) in last_name.char_indices() { // can be used instead of .chars().enumerate()
        println!("{} = {}", i, c);
    }
    
    println!();
    println!("{} has {} vowels in it.", last_name, count_vowels(&last_name));
}

fn count_vowels(s: &str) -> i32 {  // &str is more flexible here
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut result = 0;
    
    for c in s.chars() {
        if vowels.contains(&c) {
            result += 1;
        }
    }
    result
}

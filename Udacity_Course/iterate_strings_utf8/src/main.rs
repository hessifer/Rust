fn print_each_char(s: &str) {
    for c in s.chars() {
        print!("{}", c);
    }
}

fn print_vowel(s: &str) {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for c in s.chars() {
        if vowels.contains(&c) {
            print!("{}", c);
        }
    }
}

fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for c in s.chars() {
        if vowels.contains(&c) {
            count += 1;
        }
    }
    count
}

fn get_vowels(s: &str) -> Vec<char> {
    let mut results = Vec::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for c in s.chars() {
        if vowels.contains(&c) {
            results.push(c);
        }
    }
    results
}

fn print_constants(s: &str) {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for c in s.chars() {
        if !vowels.contains(&c) {
            print!("{}", c);
        }
    }
}

fn main() {
    print_each_char("Hello, world!");
    println!();
    print_vowel("Hello, world!");
    println!();
    print_constants("Hello, world!");
    println!();
    println!("The number of vowels found in \"Damaris\" is {}", count_vowels("Damaris"));
    println!();
    let vowels_in_damaris = get_vowels("Damaris");
    dbg!(vowels_in_damaris);
}

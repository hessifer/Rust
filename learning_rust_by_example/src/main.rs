use std::io;

fn main() {
    // Data Types
    // data_types();
    // take_input();
    // if_else();
    get_larger_number();
}

fn data_types() {
    // two types of data types
    // scalar and compound
    // scalar is a single value
    // compound is multiple values

    // integer type - number without a fractional component
    // integer types in Rust
    // i8 or u8 -> 8 bits signed or unsigned (positive only)
    // i16 or u16
    // i32 or u32
    // i64 or u64
    // i128 or u128
    // isize or usize -> arch (bits is based on your system hw)

    // signed values -(2^n-1) to (2^n-1) - 1  -> n is the number of bits used
    // i8 can store numbers from -(2^7) to 2^7 - 1 = -128 to 127
    // unsigned values 0 to 2^n - 1
    // u8 can store numbers from 0 to (2^8 - 1) = 255


    // integer overflow
    // u8 can store up to 255
    let no_overflow:u8 = 255;
    let overflow:u8 = 256; // here we overflow the max value by one resulting a value of 0 being returned

    println!("No Overflow Results: {}", no_overflow);
    println!("Overflow Results: {}", overflow);

    // Rust has two primitive types for floating points (f32 & f64)
    let new_float:f32 = 256.87;
    println!("Float Value: {}", new_float);

    // Boolean -> can have 1 of 2 possible values (true or false)
    let is_ready:bool = true;

    if is_ready {
        println!("Ok it's true!");
    }

    // Char
    let first_initial:char = 'c';
    println!("My names starts with the letter '{}'", first_initial);

    // Constant
    // Similar to a immutable variable; however, different
    // declare with const keyword and must use type annotation
    // const can be declared in global or local scope, variables can't
    // be declared in global scope
    const MAX_AGE:i8 = 21;
    println!("Drinking Age: {}", MAX_AGE);

    // Strings {str_ptr, length, capacity}
    // Rust has only one string type in the 'core' language -> string slice (&str)
    // The 'String' type is provided by Rust's STL
    // 'String' is growable, mutable, UTF-8 encoded
    let name:String = String::from("charles");
    let mut lname = String::new();
    lname = String::from("cookie");
    let mut word:String = String::with_capacity(25);
    word.push_str("jeep");
    let msg:&str = "hello";

    println!("Name: {}", name);
    println!("Message: {}", msg);
    println!("Food: {}", lname);

    println!("Number of bytes being consumed: {}", word.len());
    println!("Number of bytes allowed: {}", word.capacity());

    // typecasting -> use 'as' keyword
    let a:i32 = 10;
    let b:i64 = 128;
    let c:i64 = a.into(); // or 'a as i64'

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
}

fn take_input() {
    let mut a = String::new();
    let mut b = String::new();
    let mut my_msg = String::new();

    println!("Enter a String:");
    io::stdin().read_line(&mut a).expect("Failed");
    println!("Input: {}", a);

    // Typecasting as all input is of type String
    println!("Enter a Number:");

    // Get input from user
    io::stdin().read_line(&mut b).expect("Failed");

    // typecast string to i32
    let c:i32 = b.trim().parse().expect("Failed");
    println!("Entry: {}", b);


    // trimming new line
    println!("Enter a new string: ");
    io::stdin().read_line(&mut my_msg).expect("Failed");
    println!("Hello {}", my_msg.trim());
}

fn if_else() {
    // if statements do not need '()' like in other languages
    let a:i8 = 6;

    if a % 2 == 0 { // single line if statements require '{}'
        println!("Even");
    } else {
        println!("Odd");
    }

    let mut answer = String::new();
    println!("Are your friends coming to the movie [y/n]: ");
    io::stdin().read_line(&mut answer).expect("Failed"); // if you call this again answer will be appended!!! use 'clear()'

    // convert String to Vector of Chars (to access indices)
    let char_vec: Vec<char> = answer.trim().chars().collect(); // throw away the new line

    if char_vec[0] == 'y' {
        println!("Awesome, see you guys there!");
    } else {
        println!("Oh well, more popcorn for us!");
    }
}

fn get_larger_number() {
    // ask for two numbers and returns the largest number
    println!("Please enter a number: ");
    let mut entry1 = String::new();
    io::stdin().read_line(&mut entry1).expect("Failed");
    let num1:i64 = entry1.trim().parse().expect("Failed");

    println!("Please enter a number: ");
    let mut entry2 = String::new();
    io::stdin().read_line(&mut entry2).expect("Failed");
    let num2:i64 = entry2.trim().parse().expect("Failed");

    if num1 > num2 {
        println!("Larger Number: {}", num1);
    } else {
        println!("Larger Number: {}", num2);
    }

}

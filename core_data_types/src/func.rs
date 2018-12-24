#![allow(dead_code)]

// function with no parameters
pub fn say_hello() {
    println!("Hello!");
}

// function with a single parameter
pub fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true
    }
    return false
}

// function that takes two parameters
pub fn sum_of_two_nums(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

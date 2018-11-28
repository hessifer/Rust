// use the mem library that is not part of the prelude
use std::mem;

fn main() {
    let a:u8 = 123; // 8 bit unsigned integer
    println!("a = {}", a); // interpolation

    // mutable variable
    let mut b:i8 = 0;
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);
    // use mem::size_of_val() to display size in bytes, below should be 1 byte give 8 bit integer
    println!("the size of b in bytes is {}", mem::size_of_val(&b));
}
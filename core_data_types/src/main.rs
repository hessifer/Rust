// use the mem library that is not part of the prelude
use std::mem;

fn main() {
    // INTEGER

    // valid integer types i8 u8 i16 u16 i32 u32 i64 u64
    // isize or usize (get you the size of the pointer)
    let a:u8 = 123; // 8 bit unsigned integer
    println!("a = {}", a); // interpolation

    // mutable variable
    let mut b:i8 = 0;
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);
    // use mem::size_of_val() to display size in bytes, below should be 1 byte give 8 bit integer
    println!("the size of b in bytes is {}", mem::size_of_val(&b));

    // CHAR
    let d:char = 'x'; // largest unicode size (4 bytes)
    println!("Your char is {}", d);

    // ISIZE / USIZE (depends on your arch) let z:isize = 123; // variable z size is the same as the memory address for this specific OS let size_of_z = mem::size_of_val(&z); println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8); // float let e:f64 = 2.5; // double-precision, 8 bytes or 64 bits, f64 println!("e = {}, size = {} bytes", e, mem::size_of_val(&e)); let f:f32 = 2.5; // double-precision, 8 bytes or 64 bits, f32 println!("f = {}, size = {} bytes", f, mem::size_of_val(&f)); }
use std::mem;

fn main() {
    // integers (i8, u8, i16, u16, i32, u32, i64, u64, isize, usize)
    let a:u32 = 123;
    let b:u8 = 5;
    let c:i64 = 409654;
    let d:usize = 1234;

    let a_size_in_bytes: usize = mem::size_of_val(&a);
    println!("a = {}, has {} bytes, on a {}-bit os", a, a_size_in_bytes, a_size_in_bytes / 8);

    let b_size_in_bytes:usize = mem::size_of_val(&b);
    println!("b = {}, has {} bytes, on a {}-bit os", b, b_size_in_bytes, b_size_in_bytes / 8);

    let c_size_in_bytes:usize = mem::size_of_val(&c);
    println!("c = {}, has {} bytes, on a {}-bit os", c, c_size_in_bytes, c_size_in_bytes / 8);

    let d_size_in_bytes:usize = mem::size_of_val(&d);
    println!("d = {}, has {} bytes, on a {}-bit os", d, d_size_in_bytes, d_size_in_bytes / 8);


    // chars


    // floats



    // bool
}

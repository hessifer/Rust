use std::mem;

fn main() {
    // integers (i8,u8,i16,u16,i32,u32,i64,u64,isize,usize)
    let a:u32 = 123;
    let b:u8 = 5;
    let c:i64 = 409654;
    let d:usize = 1234;

    let a_size_in_bytes: usize = mem::size_of_val(&a);
    println!("a = {}, has {} bytes, a {}-bit address", a, a_size_in_bytes, a_size_in_bytes * 8);

    let b_size_in_bytes:usize = mem::size_of_val(&b);
    println!("b = {}, has {} bytes, a {}-bit address", b, b_size_in_bytes, b_size_in_bytes * 8);

    let c_size_in_bytes:usize = mem::size_of_val(&c);
    println!("c = {}, has {} bytes, a {}-bit address", c, c_size_in_bytes, c_size_in_bytes * 8);

    let d_size_in_bytes:usize = mem::size_of_val(&d);
    println!("d = {}, has {} bytes, a {}-bit address", d, d_size_in_bytes, d_size_in_bytes * 8);


    // chars
    let z:char = 'c';
    let z_size_in_bytes:usize = mem::size_of_val(&z);
    println!("z = {}, has {} bytes, a {}-bit address", z, z_size_in_bytes, z_size_in_bytes * 8);

    // floats
    let g:f32 = 2.1;
    let h:f64 = 6.9487;
    let g_size_in_bytes:usize = mem::size_of_val(&g);
    let h_size_in_bytes:usize = mem::size_of_val(&h);
    println!("g = {}, has {} bytes, a {}-bit address", g, g_size_in_bytes, g_size_in_bytes * 8);
    println!("h = {}, has {} bytes, a {}-bit address", h, h_size_in_bytes, h_size_in_bytes * 8);


    // bool
    let is_home:bool = true;
    let is_home_size_in_bytes:usize = mem::size_of_val(&is_home);
    println!("is_home = {}, has {} bytes, a {}-bit address", is_home, is_home_size_in_bytes,
             is_home_size_in_bytes * 8);

    if is_home {
        println!("You are home!");
    }
}

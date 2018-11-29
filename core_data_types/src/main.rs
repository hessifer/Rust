use std::mem;

fn core_data_types() {
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

fn operators() {
    // arithmetic

    let mut a:i32 = 2 + 3 * 4;
    println!("{}", a);

    a = a + 1; // no support for post/pre increment -- ++
    a *= a;

    println!("{}", a);

    let a_cubed:i32 = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5; // floating point value of type f64 assigned to b
    let b_cubed = f64::powi(b, 3); // b cubed to the power of an integer
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // b cubed to the power of PI

    println!("b = {}", b);
    println!("{} cubed is {}", b, b_cubed);
    println!("{} to the power of PI is {}", b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR, & AND, ^ XOR, ! NOR
    println!("1|2 is {}", c); // 01 OR 10 = 11 = the decimal value 3

    let d = 4 | 6; // 100 OR 110 = 110 = the decimal value 6
    println!("4|6 is {}", d);

    let two_to_10 = 1 << 10; // << shift operator
    println!("2^10 = {}", two_to_10);

}

fn main() {
    core_data_types();
    operators();
}

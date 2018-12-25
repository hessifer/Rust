#![allow(dead_code)]
mod sh;
mod func;
mod loopy;
mod calc_fib_term;
mod data_structures;
mod enums;
use std::mem;

const FAVORITE_NUMBER:u8 = 5; // no fixed of address

static mut FAVORITE_DAY:u8 = 2; // allows access to address of variable (UNSAFE operation (unsafe)

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

    // two primitive compound types: tuple & array
    let a_tup: (i32, f64, u8) = (500, 6.4, 1); // with type annotation
    let b_tup = (300, 3.4, 5); // with-out optional type annotation

    // destructuring a tuple
    let (a, b, c) = a_tup;
    let (d, e, f) = b_tup;

    println!("The value of a is {}", a);
    println!("The value of b is {}", b);
    println!("The value of c is {}", c);
    println!("The value of d is {}", d);
    println!("The value of e is {}", e);
    println!("The value of f is {}", f);
    println!("The third value in b_tup is: {}", b_tup.2);

    // arrays
    let my_array = [2, 3, 4, 5, 6];
    println!("The 3rd element of my_array is {}", my_array[2]);


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

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // should be true
    println!("{} < 4.0 = {}", std::f64::consts::PI, pi_less_4);

}

fn scope_and_shadowing() {
    let a = 123;

    // let a = 999; // the latest declaration overrides the first (produces warning)

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 53;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
}

fn declaring_and_using_constants() {
    // display favorite number constant
    println!("Favorite Number: {}", FAVORITE_NUMBER);
}

fn main() {
    core_data_types();
    operators();
    scope_and_shadowing();
    declaring_and_using_constants();

    // for unsafe operations use 'unsafe' block
    unsafe {
        println!("Favorite Day: {}", FAVORITE_DAY);
        // we can change value of our 'static' variable FAVORITE_DAY
        FAVORITE_DAY = 3;
        println!("Modified Favorite Color Day: {}", FAVORITE_DAY);
    }

    // use sh.rs module
    sh::stack_and_heap_allocation();


    // working with functions
    func::say_hello();
    if func::is_even(5) {
        println!("The number 5 is even.");
    } else {
        println!("The number 5 is odd.");
    }

    let answer = func::sum_of_two_nums(11, 12);
    println!("The sum of {} and {} is {}", 11, 12, answer);


    // working with loops
    loopy::repeat_forever();
    loopy::repeat_until(5);
    let mut numbers = [20, 21, 22, 23, 24, 25];
    loopy::show_evens(&mut numbers);

    // calc fibonacci term
    let result = calc_fib_term::calc_fib_term(5);
    println!("The 5th fibonacci value is: {}", result);

    // Data Structures
    let my_point = sh::Point { x: 5.6, y: 7.8 }; // our struct is defined in sh.rs
    let point_a = sh::Point { x: 1.6, y: 4.8 }; // our struct is defined in sh.rs
    let point_b = sh::Point { x: 0.6, y: 9.8 }; // our struct is defined in sh.rs
    let my_line = sh::Line { start: point_a, end: point_b };
    data_structures::display_point(my_point);
    data_structures::display_line(my_line);


    // Enums
    enums::enums();
}

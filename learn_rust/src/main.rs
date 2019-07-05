mod variables;
mod types;
mod numeric_operations;
mod gcd;
mod arrays;


fn main() {
    println!("{}", termion::clear::All); 
    variables::learn_variables();
    types::rust_types_take_one();
    numeric_operations::numeric_operations();
    println!("GCD of 16384 and 8192 is: {}", gcd::gcd(16384, 8192));
    println!("GCD of 8192 and 16384 is: {}", gcd::gcd(8192, 16384));
    arrays::arrays_rust();
}
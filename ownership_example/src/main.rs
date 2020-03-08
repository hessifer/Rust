fn main() {
    let s = "hello"; // allocated on the stack, immutable string literal
    let mut msg = String::from("Good Day Mate!"); // allocated on the heap, mutable
    msg.push_str(" Aloha!"); // appeands to our String
    
    println!("{}", s);
    println!("{}", msg);

    // example of simple copy of two string literal variables on the stack
    let x = 5;
    let y = x; // known fixed size placed on stack

    // example of two Strings on the heap
    let s5 = String::from("Charles");
    // let s6 = s5; // s5 has been moved to s6

    // In the example above, this works differently on variables allocated on the heap. String
    // variables are made up of three parts. 
    // 1. pointer to the memory that holds the content
    // 2. length, how much memory in bytes the contents of String is currently using 
    // 3. capacity, the total amount of memory in bytes the String has received from the OS
    takes_ownership(s5);

    let t = 5;
    makes_copy(t);
    println!("{}", x);
    println!("{}", y);
    println!("{}", t); // copy made given t is a primary data type int

} // out of scope, Rust will call drop() on variables not yet cleaned up 

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // here some_string goes out of scope and 'drop' is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // here, some_integer goes out of scope.

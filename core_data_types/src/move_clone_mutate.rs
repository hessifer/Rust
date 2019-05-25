use std::clone::Clone;
use std::string::ToString;

pub fn by_moving() {
    /*
    * straight-forward and clear
    * uses only immutable data, best practice is to reduce stateful behavior
    * reuses memory allocated for hello, very performant
    */

    let hello = "hello ".to_string();
    let world = "world!";

    // Move hello into a new variable
    let hello_world = hello + world; // hello can no longer be used

    println!("{}", hello_world);
}

pub fn by_cloning() {
    /*
    * clones the allocated string into a temp object, new memory allocation
    * creates redundant memory allocation at runtime.
    */

    let hello = "hello ".to_string();
    let world = "world!";

    // Create a copy of hello and move it into a new variable
    let hello_world = hello.clone() + world; // hello can still be used

    println!("{}", hello_world);
}

pub fn by_mutating() {
    /*
    * this is the stateful way of doing this
    * variable is muttable and performance similar to moving
    * Rust prefers that you move data in order to create new variables
    * only use state if you really need mutable data or want to introduce in small
    * and manageable context
    */

    let mut hello = "hello ".to_string();
    let world = "world!";

    hello.push_str(world); // hello modified in place and usable
    println!("{}", hello);
}
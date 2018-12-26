pub fn var_copies() {
    let s1 = String::from("ok"); // data stored on heap, (ptr, length, capacity on stack)
    let s2 = s1; // not a complete copy, ptr, length, capacity is copied, but not data at addr.

    // shallow copy - ptr, length, capacity is copied but not the actual data
    //                can lead to 'double free' issue causing potential sec vulns.
    // deep copy - copies ptr, length, capacity, and data. (this is not done by default in Rust)
    // clone - used in Rust to perform deep copy
    // drop - Rust uses this special function for deallocating resources at the end of their lifetime
    //        or in this guess once ownership is gone. Similar to Resoruce Acquisition Is Initialization (RAII)
    //        in C++.

    // if we try and access s1 here we will get an error as Rust has already deallocated s1 to prevent
    // issues associated with shallow copy, double free, for example.



    // println!("s1: {}", s1); // will throw error 'use of moved value' memory no longer valid
    println!("s2: {}", s2);

    // we can use clone() to copy heap data as well as stack data but this is an expensive op.
    let msg1 = String::from("today");
    let msg2 = msg1.clone(); // copy heap data as well

    println!("msg1: {}", msg1);
    println!("msg2: {}", msg2);


    // IMPORTANT NOTE: types that have a known fixed size at compile time are stored on the stack
    //                 and not on the heap. The following is valid.

    let x = 5; // known fixed size i32
    let y = x; // no need for clone

    println!("x: {}, y: {}", x, y);
}


pub fn takes_ownership(some_string: String) { // will perform move (some_string in scope)
    // passing a variable to a function will move or copy, just as assignment does.
    println!("{}", some_string);
} // some_string out of scope and drop() is called, memory is freed


pub fn makes_copy(some_integer: i32) { // some_integer in scope
    println!("{}", some_integer);
} // some_integer out of scope (on stack so nothing special happens)

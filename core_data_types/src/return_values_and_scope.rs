pub fn gives_ownership() -> String {
    // will move its return value into the function that calls it
    let some_string = String::from("hello");

    some_string // some_string is returned and moves out of calling function
}


pub fn takes_and_gives_back(a_string: String) -> String { // a_string in scope
    // takes a String and then returns one
    a_string // a_string is returned and moves out to the calling function
}

// all this coming in moving out can be overkill, this is why Rust has the concept of references (&a_string)
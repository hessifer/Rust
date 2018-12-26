pub fn ownership_example_1() {
    let s = "sup"; // string literal stored on stack (immutable)
    let mut msg = String::from("wazzzup!"); // String store on heap (we may not know the values, mutable)

    println!("String Literal: {}", s);

    // example of mutating String msg
    msg.push_str(", peeps"); // push_str() appends a literal to a String
    println!("{}", msg); // prints 'wazzzup!, peeps'
}
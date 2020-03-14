use std::env;

fn main() {
    let name = env::args().skip(1).next();

    match name {
        Some(n) => println!("Hi there {}!", n),
        None => panic!("Didn't recieve any name?")
    }

    // if_else_no_value_returned
    let result = if 1 == 2 {
        "Nothing makes sense";
    } else {
        "Sanity reigns";
    };

    // if_else_value_returned
    let r = if 1 == 2 {
        "sup"
    } else {
        "dude"
    };

    println!("Result of computation: {:?}", result);
    println!("Result of computation: {:?}", r);
}
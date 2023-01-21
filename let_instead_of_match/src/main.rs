fn main() {
    let is_option = Some("a");

    // if we only care about the Some() from Option
    // we do not need a match statement
    /*
    match my_option {
        Some(my_value) => do_something(my_value),
        _ => {}
    }
    */

    // same as above but with if let for a single match
    if let Some(my_value) = is_option {
        do_something(my_value);
    }
}

fn do_something(value: &str) {
        println!("{}", value);
}

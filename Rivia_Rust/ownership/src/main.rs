fn main() {
    // example of passing simple scalar value
    let my_num = 5;
    let num_plus_two = add_two(my_num);

    println!("{}", my_num);
    println!("{}", num_plus_two);
    println!("{}", my_num); // my_num still valid

    // example of passing String
    let s = String::from("wazzup");

    display_message(s); // s moves into display_message no Copy trait
    println!("{}", s); // this line produces compile time error, s no longer valid

    // example of borrowing
    let color = String::from("blue");

    show_color(&color); // show_color borrows color
    println!("{}", color); // color still valid

    // mutable reference at the same time of immutable reference
    let mut name = String::from("Bob");
    let s = &name;
    alter_message( &mut name); // already have immutable reference

    println!("Name: {}", name);
    println!("Value of s: {}", s);
}

fn add_two(num: i32) -> i32 {
    num + 2
}

fn display_message(text: String) {
    println!("{}", text);
}

fn show_color(color: &str) {
    println!("{}", color);
}

fn alter_message(msg: &mut String) {
    msg.push_str(" on Tuesday")
}

/*
Examples of defining functions with no parameters, with parameters. Functions
with no return value and functions with a return value.
*/

fn main() {
    // Declare our msg variable of type String
    let msg: String = String::from("Wazzzz Up!");

    // Invoke our greeting() function that takes 1 argument of type String
    greeting(msg);

    // Declare our number variables
    let num1 = 5;
    let num2 = 3;

    // Invoke sum_two_nums using 2 arguments of type i32
    println!("The sum of {} and {} is {}.", num1, num2, sum_two_nums(num1, num2));

    // Invoke say_hello with num arguments or return value
    say_hello();

    // Invoke shout_greeting using mutable reference
    let mut msg: String = String::from("wazzzup!");
    println!("{}", shout_greeting(&mut msg));
}

fn greeting(txt: String) {
    // Function to display the value of txt to STDOUT.
    println!("{}", txt);
}

fn say_hello() {
    // Function to display "hello" to STDOUT
    println!("hello");
}

fn sum_two_nums(num1: i32, num2: i32) -> i32 {
    // Function to return resulting value of an expression
    num1 + num2
}

fn shout_greeting(greeting: &mut str) -> String {
    return greeting.to_uppercase();
}


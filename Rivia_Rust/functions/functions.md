### Functions
* pervasive in Rust
* programs start with main()
* functions and variables use *snake_case* to separate words and all letters are lower case
* functions are defined with keyword **fn**
```
fn say_hello() {
        println!("Hello!");
}
```
* function declarations may occur before or after the declaration of main
* function definitions may included *parameters*, special variables that are part of the function
signature
* values passed into a function are called *arguments* and act as special variables with the scope of the function
```
fn main() {
    let greeting: String = String::from("Hello World!");
    say_hello(greeting);
}

fn say_hello(msg: String) {
    println!("{}", msg);
}
```
* **NOTE:** in the function signature you **must** annotate the type for each parameter
* inside the {} of the function is referred to as the *function block* and contains a series of statements
* In Rust, functions can end in an **expression** as opposed to a statement
```
fn add_two(num: i32) {
    num + 2
}
```
* *Statements* instructions that perform some action and do not return a value
* *Expressions* evaluate to resulting value
    * expressions do not end in a semicolon
* functions may have return values using the *return* keyword as well
* you must declare the type of the value the function will reuturn after the '->' symbol
```
fn five_times_five() -> i32 {
    5 * 5
}
```

### Comments
* Rust uses '//' at the beginning of a line to begin a single line comment
* Rust uses '/* multi-line comment*/'
```
// Rust single line comment

/*
Rust multi-line
comment.
*/
```

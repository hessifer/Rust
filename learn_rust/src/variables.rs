#[allow(dead_code)]
pub fn learn_variables() {
    // section for learning to work with variables
    const MY_FAVORITE_NUMBER :i8 = 5;  // define a constant can be in any scope
    let msg = String::from("Hello, World!");  // immutable variable of type String

    println!("{}", msg);
    println!("My Favorite Number: {}", MY_FAVORITE_NUMBER);
    println!("The Message Has a Length of: {} bytes", msg.len());
}
fn main() {
    // single if / else
    let age = 27;

    if age > 21 {
        println!("Looks like you are allowed to drive.");
    } else {
        println!("Hang in there, you will be behind the wheel in no time.");
    }

    // if / else if / else
    let my_age = 30;

    if my_age < 13 {
        println!("You are a pre-teen.");
    } else if my_age < 20 {
        println!("You are a teen.");
    } else if my_age < 40 {
        println!("You are in your prime.");
    } else {
        println!("You are over the hill.");
    }

    // using if expression to assign a value to a variable
    let num = 6;
    let is_even = if num % 2 == 0 {
        true
    } else {
        false
    };

    println!("The number {} is even: {}", num, is_even);


    // loop
    let mut counter = 0;

    loop {
        println!("Keep going until I reach a counter of 3.");
        counter += 1;

        if counter == 3 {
            break; // if we don't use break we create an infinite loop
        }
    }

    // while
    let mut count = 3;

    while count > 0 {
        println!("{}", count);
        count -= 1;
    }
    println!("Blast off!");

    // for with array
    let colors = ["yellow", "blue", "orange"];

    for color in colors.iter() {
        println!("{}", color);
    }

    // for with range
    for i in (1..4).rev() {
        println!("{}", i);
    }
    println!("Blast off!");
}

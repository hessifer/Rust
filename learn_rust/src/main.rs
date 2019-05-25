#[allow(dead_code)]
fn learn_variables() {
    // section for learning to work with variables
    const MY_FAVORITE_NUMBER :i8 = 5;  // define a constant
    let msg = String::from("Hello, World!");  // immutable variable of type String

    println!("{}", msg);
    println!("My Favorite Number: {}", MY_FAVORITE_NUMBER);
    println!("The Message Has a Length of: {}", msg.len());
}

#[allow(dead_code)]
fn numeric_operations() {
    // addition
    let sum = 5 + 5;

    // subtraction
    let difference = 10 - 5;

    // multiplication
    let product = 5 * 5;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Quotient Two Decimal Places: {:.2}", quotient);
    println!("Remainder: {}", remainder);
}

#[allow(dead_code)]
fn rust_types_take_one() {
    // Scalar type
    let num :i8 = 4;
    println!("{}", num);

    let letter :char = 'c';
    println!("{}", letter);

    let is_cool :bool = true;

    // Compound type
    let tup: (i32, f64, String) = (128, 3.14, String::from("blue"));
    println!("{:?}", tup);

    if is_cool {
        println!("{}", tup.2);
    } else {
        println!("Tuple Element 2: {}", tup.1);
    }

    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
}

#[allow(dead_code)]
fn arrays_rust() {
    // define an array of integers
    let my_num_array = [3, 4, 5, 6, 7, 7]; // array of integers [i32; 6]

    println!("{:?}", my_num_array);
    for i in my_num_array.iter() {  // return an iterable to process
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}

fn gcd(num1: i32, num2: i32) -> i32 {
    let mut greatest_common_divisor = 1;
    let mut gcd_upper_limit = num1;

    if num2 < num1 {
        gcd_upper_limit = num2;
    }

    for i in 1 ..= gcd_upper_limit {
        if num1 % i == 0 && num2 % i == 0 {
            greatest_common_divisor = i;
        }
    }
    greatest_common_divisor
}

fn main() {
   // learn_variables();
   // numeric_operations();
   // rust_types_take_one();
   // arrays_rust();
    println!("GCD of 16384 and 8192 is: {}", gcd(16384, 8192));
}

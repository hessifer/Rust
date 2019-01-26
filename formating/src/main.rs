fn main() {
    // example of formatting
    println!("{:?}", [1, 2, 3]);

    // simple_message();
    // println!("Sum: {}", sum_these(8, 2));
    println!("Difference: {}", sub_these(8, 2));

    // closure (anoymous functions) no annotated types
    let x = 5;
    let square = |x| x * x;
    println!("5 squared: {}", square(x));
    
    // closure (anoymous functions) with annotated types
    let y = 4;
    let plus_one = |y: i32| -> i32 {
        y + 1
    };
    println!("4 + 1 = {}", plus_one(y));

    let a = 10;
    let c = 5;
    let sub_two_nums = |_d: i32, _e: i32| -> i32 {
        a - c
    };
    println!("{} - {} = {}", a, c, sub_two_nums(a, c));

    // arrays
    //arrays();
    
    // tuples
    //tuples();
    
    // slices
    // slices();

    // str
    str_s();
}

// simple function
#[allow(dead_code)]
fn simple_message() {
    println!("Hello World!");
}

// func arguments
#[allow(dead_code)]
fn sum_these(num1: i8, num2: i8) -> i8 {
    num1 + num2
}

// func arguments with return
fn sub_these(num1: i8, num2: i8) -> i8 {
    return num1 - num2;
}

// playing with arrays (always immutable)
#[allow(dead_code)]
fn arrays() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let d: [i32; 3] = [1, 2, 3];

    println!("{:?}", nums);
    println!("{:#?}", d);

    println!("\nEven Numbers");
    for i in nums.iter() {
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}

// playing with tuples (immutable by default, count/size cannot change)
// fixed size of ordered list of elements of different or same data type
#[allow(dead_code)]
fn tuples() {
    let my_tuple = (1, 2.3, 'c', "Hello, World!"); // simplified struct
    let b: (i32, f64) = (55, 3.14);

    println!("{:?}", b);
    println!("Message: {}", my_tuple.3);
}


// slices (dynamically-sized reference to another data structure)
#[allow(dead_code)]
fn slices() {
    let a: [i32; 4] = [1, 2, 3, 4];
    let b: &[i32] = &a; // slice whole array
    let c: &[i32] = &a[1..3]; // slice from 2nd to 4th excluding
    let d = &a[..];
    let e = &a[..];
    
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", e);
}

// str
fn str_s() {
    let msg: &str = "Wazzzzzup!";

    println!("Message: {}", msg);
}

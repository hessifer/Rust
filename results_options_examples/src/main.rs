fn main() {
    let a = 144;
    let b = 12;
    let c = 0;
    let d = 8;
    let result1 = divide(a, b);
    let result2 = divide(d, c);
    println!("{} / {} is {:?}", a, b, divide(a, b));
    println!("{} / {} is {:?}", d, c, divide(d, c));

    // implementing a match for our Res
    match result1 {
        // Result is part of stand library so we dont need to type it below
        // Result::Ok(v) => println!("Value: {}", v),
        Ok(v) => println!("Value: {}", v),
        _ => {}
    }

    match result2 {
        Ok(v) => println!("Value: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // if you only care about 1 case
    if let Ok(v) = result1 {
        println!("val = {}", v)
    }
}


fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
}

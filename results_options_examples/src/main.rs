#[derive(Debug)]
pub enum Res<T, E> { // generics, Res can be of any type
    Thing(T),
    Error(E),
}

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
        Res::Thing(v) => println!("Value: {}", v),
        _ => {}
    }

    match result2 {
        Res::Thing(v) => println!("Value: {}", v),
        Res::Error(e) => println!("Error: {}", e),
    }
}


fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string());
    }
    Res::Thing(a / b)
}

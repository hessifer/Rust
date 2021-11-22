pub enum Res<T, E> { // generics, Res can be of any type
    Thing(T),
    Error(E),
}

fn main() {
    let a = 144;
    let b = 12;
    println!("{} / {} is {}", a, b, divide(a, b));
}


fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string());
    }
    Res::Thing(a / b)
}

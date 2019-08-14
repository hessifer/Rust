fn add(a: i32, b: i64) -> i32 {
    a + (b as i32)
}

fn main() {
    let num1 = 1;
    let num2 = 2;

    println!("{} + {} = {}", num1, num2, add(num1, num2));
}

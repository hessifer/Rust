fn main() {
    let num1 = 45;
    let num2 = 1456;
    let num3 = 1024;
    
    println!("The largest number of 45, 1456, and 1024 is {}.", highest(num1, num2, num3));
}

fn highest(num1: i32, num2: u32, num3: i64) -> i32 {
    let mut result = num1;
    
    if num2 as i32 > result {
        result = num2 as i32;
    }
    
    if num3 as i32 > result {
        result = num3 as i32;
    }
    result
}

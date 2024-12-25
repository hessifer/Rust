fn add_with_lifetimes(i: &i32, j: &i32) -> i32 {
    *i + *j
}

fn mul_with_lifetimes(x: &i32, y: &i32) -> i32 {
    *x * *y
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    let mul_res = mul_with_lifetimes(&a, &b);

    println!("{}", res);
    println!("{} * {} = {}", a, b, mul_res);
}

#[allow(dead_code)]
pub fn gcd(num1: i32, num2: i32) -> i32 {
    let mut greatest_common_divisor = 1;
    let mut gcd_upper_limit = num1;

    if num2 < num1 {
        gcd_upper_limit = num2;
    }

    for i in 1 ..= gcd_upper_limit { // inclusive range on both ends (..=)
        if num1 % i == 0 && num2 % i == 0 {
            greatest_common_divisor = i;
        }
    }
    greatest_common_divisor
}
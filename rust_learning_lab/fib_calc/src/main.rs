const FIB_ZERO: u64 = 0;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    for i in 1..9 {
        println!("{}:{}", i, fib(i));
    }
}

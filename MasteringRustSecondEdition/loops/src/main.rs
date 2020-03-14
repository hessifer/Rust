fn silly_sub(a: i32, b: i32) -> i32 {
    let mut result = 0;

    'increment: loop {
        if result == a {
            let mut dec = b;
            loop {
                if dec == 0 {
                    // breaks directly out of 'increment loop
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}
fn main() {
    // loop contructs can be done in three ways in Rust
    // loop, while, for
    let mut x = 4;

    loop {
        if x < 0 {
            break;
        }
        println!("{} more runs to go", x);
        x -= 1;
    }

    // sill_sub called
    println!("{}", silly_sub(5, 10));

    // while loop
    let mut e = 10;

    while e > 0 {
        println!("{} more runs to go", e);
        e -= 1;
    }

    // for loop (works on iterators)
    print!("Normal Ranges: ");
    for i in 0..10 {
        print!("{}, ", i)
    }

    println!();
    print!("Inclusive ranges: ");
    for i in 0..=10 {
        print!("{}, ", i);
    }
}

fn main() {
    let numbers_tuple: (i32, i32, i32) = (10, 13, 5);
    let (x, y, z) = numbers_tuple;

    println!("x -> {}", x);
    println!("y -> {}", y);
    println!("z -> {}", z);

    for num in [numbers_tuple.0, numbers_tuple.1, numbers_tuple.2].iter() {
        println!("{}", num);
    }
}

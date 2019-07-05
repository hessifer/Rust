#[allow(dead_code)]
pub fn rust_types_take_one() {
    // Scalar type (singular value/type)
    let num :i8 = 4;
    println!("Integer: {}", num);

    let letter :char = 'c';
    println!("Character: {}", letter);

    let is_cool :bool = true;

    // Compound type (multiple values / types)
    let tup: (i32, f64, String) = (128, 3.14, String::from("blue"));
    println!("Tuple: {:?}", tup);

    if is_cool {
        println!("Tuple Element 3: {}", tup.2);
    } else {
        println!("Tuple Element 2: {}", tup.1);
    }

    let (x, y, z) = tup;
    println!("Unpack / Destructure the Tuple: {}, {}, {}", x, y, z);
}
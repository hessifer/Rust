fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6]; // create vector with marco
    let v2 = vec![8, 9, 0];

    println!("{:?}", longer_vector(&v1, &v2)); // pass reference to our vectors
}

fn longer_vector<'a>(x: &'a[i32], y: &'a[i32]) -> &'a[i32] {
    if x.len() > y.len() { x } else { y } // single line goodness
}

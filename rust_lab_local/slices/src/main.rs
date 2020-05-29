fn main() {
    let a = [1, 2, 3]; // create an array
    let v = vec![4, 5, 6]; // create a vector
    let v_slice = &v[..]; // create a slice of vector v

    only_reference_to_array(&a);
    only_reference_to_vector(&v);
    // range not required as a reference to a vector or array 'coerces'
    // to a &str (string slice)
    reference_to_either_array_or_vector(&a[..]);
    // reference_to_either_array_or_vector(&a);
    reference_to_either_array_or_vector(&v[..]);
    // reference_to_either_array_or_vector(&v);
    reference_to_either_array_or_vector(&v_slice[0..1]);
}

// can only take an array of 3 elements
fn only_reference_to_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

// can only take a vector of i32
fn only_reference_to_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

// can take reference to either a vector or an array
// any slice of i32 will work
fn reference_to_either_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}
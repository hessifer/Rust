pub fn basic_vector() {
    /*
    * Basic vector that is stored on the heap. This function is a public function
    * so that it can be used outside of this file and does not take any arguments
    * or have a return value.
    */

    // allocate a new vector on the heap of type String
    let mut my_colors: Vec<String> = Vec::new();

    my_colors.push(String::from("yellow"));
    println!("{:?}", my_colors);


    // allocate a new vector on the heap of type i32
    let mut my_numbers: Vec<i32> = Vec::new();

    for i in 0 .. 100 {
        my_numbers.push(i);
    }

    println!("Numbers: {:?}", my_numbers);
}

pub fn vector_as_arguments(nums: &Vec<i32>) {
    /*
    * function that accepts a vector as an argument, but has no
    * return value.
    */

    // print the size of the passed in vector
    println!("Your vector has a size of: {}", nums.len());

    // invoke another function from within a function
    for i in nums.iter() {
        if is_even(*i) {
            print!("{} ", i);
        }
    }
    println!();
}

pub fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}
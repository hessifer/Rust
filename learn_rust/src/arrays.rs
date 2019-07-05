#[allow(dead_code)]
pub fn arrays_rust() {
    // define an array of integers
    let my_num_array = [3, 4, 5, 6, 7, 7]; // array of integers [i32; 6]

    println!("Integer array: {:?}", my_num_array);
    println!("Even Numbers in Integer Array:");
    for i in my_num_array.iter() {  // return an iterable to process
        if i % 2 == 0 {
            println!("{}", i);
        }
    }
}
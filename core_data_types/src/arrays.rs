pub fn find_evens(nums: &[i32]) {
    // takes a ref to an array of i32 and prints the even values
    for num in nums.iter() {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}
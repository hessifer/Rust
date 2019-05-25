pub fn find_evens(nums: &[i32]) {
    // takes a ref to an array of i32 and prints the even values
    for num in nums.iter() {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}

pub fn find_odds(nums: &[i32]) -> Vec<i32> {
    // takes a ref to an array of i32 and returns a vector of odd numbers
    let mut results: Vec<i32> = Vec::new();

    for num in nums.iter() {
        if num %2 != 0 {
            results.push(*num);
        }
    }
    // println!("Odds: {:?}", results);
    results
}
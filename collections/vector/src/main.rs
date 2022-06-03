fn main() {
    // Vector of type String
    /*let mut colors: Vec<String> = Vec::new();
    colors.push(String::from("orange"));
    colors.push(String::from("yellow"));
    colors.push(String::from("blue"));
    colors.push(String::from("red"));
    colors.push(String::from("green"));
    */
    let colors: Vec<String> = vec![String::from("orange"), String::from("yellow"), String::from("blue"), String::from("red"), String::from("green")];
    // Vector of type u32
    let nums: Vec<u32> = vec![4, 5, 6, 8, 1, 23, 25];

    for color in colors.iter() {
        println!("{}", color);
    }

    for num in nums.iter() {
        if num % 2 == 0 {
            println!("{} is an even number.", num);
        }
    }
}

use std::io;
use std::io::Write;
use std::mem;

/*
  - get 10 numbers from user and store them in a vector
  - create an is_even() function
  - create a sum() function
  - create a avg() function
  - create a bytes_used() function
  - display numbers from least to greatest
  - display numbers from greatest to least
*/

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for _ in 1..=10 {
        let mut input = String::new();

        // get input from user
        print!("Please enter a number: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        // convert number and store in numbers
        numbers.push(input.trim().parse::<i32>().unwrap());
    }

    for num in &numbers {
        if is_even(*num) {
            println!("{}", num);
        }
    }

    println!("\nTotal: {}", sum_numbers(&numbers));
    println!("Average: {}", avg_numbers(&numbers));
    println!("Total Bytes: {}", bytes_used(&numbers));
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn sum_numbers(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;

    for num in numbers {
        total += num;
    }
    total
}

fn avg_numbers(numbers: &Vec<i32>) -> f64 {
    f64::from(sum_numbers(numbers)) / (numbers.len() as f64)
}

fn bytes_used(numbers: &Vec<i32>) -> usize {
    mem::size_of_val(&numbers[0]) * numbers.len()
}

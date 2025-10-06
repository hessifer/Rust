fn calculate_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }
    sum
}

fn find_largest_number(numbers: &[i32]) -> i32 {
    let mut largest_number = numbers[0];

    for &number in numbers.iter().skip(1) {
        if number > largest_number {
            largest_number = number;
        }
    }
    largest_number
}

fn find_smallest_number(numbers: &[i32]) -> i32 {
    let mut smallest_number = numbers[0];

    for &number in numbers.iter().skip(1) {
        if number < smallest_number {
            smallest_number = number;
        }
    }
    smallest_number
}

fn find_prime_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut prime_numbers = Vec::new();

    for &number in numbers {
        if number <= 1 {
            continue;
        }

        let mut is_prime = true;
        for i in 2..=number.isqrt() {
            if number % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            prime_numbers.push(number);
        }
    }
    prime_numbers
}

fn find_even_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut even_numbers = Vec::new();

    for &number in numbers {
        if number % 2 == 0 {
            even_numbers.push(number);
        }
    }
    even_numbers
}

fn find_odd_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut odd_numbers = Vec::new();

    for &number in numbers {
        if number % 2 != 0 {
            odd_numbers.push(number);
        }
    }
    odd_numbers
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(calculate_sum(&numbers), 15);
    }

    #[test]
    fn test_find_largest_number() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_largest_number(&numbers), 5);
    }

    #[test]
    fn test_find_smallest_number() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_smallest_number(&numbers), 1);
    }

    #[test]
    fn test_find_prime_numbers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_prime_numbers(&numbers), vec![2, 3, 5]);
    }

    #[test]
    fn test_find_even_numbers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_even_numbers(&numbers), vec![2, 4]);
    }

    #[test]
    fn test_find_odd_numbers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(find_odd_numbers(&numbers), vec![1, 3, 5]);
    }
}

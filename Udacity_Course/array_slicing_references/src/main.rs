fn array_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;

    for num in nums.iter() {
        sum += num;
    }
    sum
}

fn array_positives(nums: &[i32]) -> Vec<i32> {
    let mut positives = Vec::new();

    for num in nums.iter() {
        if *num > 0 {
            positives.push(*num);
        }
    }
    positives
}

fn main() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let sum = array_sum(&nums);
    let positives = array_positives(&nums);
    dbg!(sum);
    dbg!(positives);
}

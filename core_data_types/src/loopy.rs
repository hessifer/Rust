#![allow(dead_code)]
#![allow(unused_variables)]

pub fn repeat_forever() {
    loop {
        println!("forever and ever...");
        break; // comment out this line to loop forever and ever :`)
    }
}

pub fn repeat_until(count: i32) {
    let mut lap = 0;
    while lap < count {
        println!("one lap completed");
        lap += 1; // no post increment :-(
    }
}

pub fn for_with_enum() {
    for (pos, x) in (1..11).enumerate() {
        println!("Pos: {} Value: {}", pos, x);
    }
}

pub fn show_evens(nums: &mut [i32]) {
    for num in nums.iter() {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}
// stepper object
pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    // provide functionality / traits for objects
    // requires type and function next
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    // stepper iterator
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    loop {
        match st.next() {
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }

    let mut step = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };

    // using a while let as we only care about the Some(v)
    while let Some(v) = step.next() {
        println!("while, {}", v);
    }

    let new_step = Stepper {
        curr: 0,
        step: 2,
        max: 10,
    };

    // for loop with an Iterator
    for i in new_step {
        println!("for loop {}", i);
    }

    let mut n = 0;
    // infinite loop
    println!("Infinite Loop");

    loop {
        n += 1;
            if n > 10 {
              break;
            }
        println!("{}", n);
    }
    println!("All done!");

    println!("while loop");

   // while loop
    let mut m = 0;

    while m < 10 {
        m += 1;
        println!("{}", m);
    }

    println!("for loop");

    // for loop
    for i in 0..10 { // range object
        println!("{}", i);
    }
}

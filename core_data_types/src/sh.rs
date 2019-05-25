#![allow(dead_code)]
use std::mem;

// public struct for use outside this file
pub struct Point {
    pub x: f64,
    pub y: f64
}

pub struct Line {
    pub start: Point,
    pub end: Point
}

fn origin() -> Point {
    Point{x: 0.4, y: 3.4}  // no semi-colon here
}

pub fn stack_and_heap_allocation() {
    // allocate a variable and it's value on the stack
    let x = 5; // stack is Last In First Out, fast but limited in storage

    // allocate on the heap
    let y = Box::new(10); // y is stored on the stack with an address that points to the value

    // stored on the heap
    println!("{}", x); // pop value of x off the stack
    println!("{}", *y); // use the dereference symbol (*) to get value stored at address on heap

    let p1 = origin(); // stack allocated
    let p2 = Box::new(origin()); // heap allocated (pointer to address where value is stored) (8 bytes on the stack and 16 bytes on the heap)

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // address of (&)
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // let p3 have the value of p2 on the stack by using a pointer (dereference)
    let p3 = *p2; // p3 is stored on the stack using the value stored in p2 (16 bytes on the stack)
    println!("p3 takes up {} bytes", mem::size_of_val(&p3)); // address of (&)
    println!("x: {}", p3.x);
    println!("y: {}", p3.y);


}
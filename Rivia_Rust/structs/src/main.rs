// related data
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // calculate the area of a rectangle
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // check if current instance of Rectangle can fit into another
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        // associated function, related to Rectangle but does not need an instance of it
        Rectangle { width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {} square pixels.",
            rect1.area()); // pass a reference of rect1 so we can maintain ownership

    // let's debug rect1
    println!("rect1 is {:?}", rect1); // use Debug trait to print instance of Rectangle

    // another debug but pretty print
    println!("rect1 is {:#?}", rect1); // use Debug trait to print instance of Rectangle

    // use associated function
    let sq = Rectangle::square(3);
    println!("Square Size: {:#?}", sq);

    // use can_hold
    let rect2 = Rectangle { width: 10, height: 30};

    println!("rec2 can fit into rec1: {}", rect1.can_hold(&rect2));
}


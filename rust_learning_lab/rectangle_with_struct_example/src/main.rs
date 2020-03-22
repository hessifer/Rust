/*
Structs allow the developer to create custom types
that keep associated pieces of data connected to each
other and name exach piece to make the code clear.

methods - let you specify the behavior(s) that instances
of your structs have.

associated functions - let you namespace functionality that
is particular to your struct without having an instance of it
available.
*/

#[derive(Debug)] // implement Debug trait for Display
struct Rectangle {
    width: u32,
    height: u32,
}

/*
NOTE: it is valid syntaxt to have multple 'impl' blocks
for a struct. For example when dealing with generic types
and traits. No need to do so below.
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
    define associated function, does not work with an
    instance of struct.
    no need to pass self as parameter, but the function
    is still associated to the struct
    an example of an associated function is String::from
    Associated functions are often used for constructors
    that will return a new instance of the struct.
    */
    fn square(size: u32) -> Rectangle {
        // takes one dimension and uses that as both width and height (square)
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // create an instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // call associated function square from Rectangle
    let sq = Rectangle::square(3);

    // user the area() method on the Rectangle object rect1
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // use output format Debug {:?} small structs or
    // {:#?} for larger structs
    println!("Contents of rect1: {:#?}", rect1); // pretty-print

    // can_hold example
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    // associated function display
    println!("Squared Rectangled: {:#?}", sq);
}

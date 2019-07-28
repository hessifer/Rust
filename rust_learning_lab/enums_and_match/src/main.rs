#[derive(Debug)]
pub struct Bed {
    size: i32,
    count: u32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Lounge(i32, String),
}

fn main() {
    use self::Room::*;

    let t = Bedroom(Bed { size: 50, count: 3 });
    let l = Lounge(5, "big".to_string());

    println!("Hello from the {:?}.", t);

    match t {
        Kitchen(n) => println!("The room is a kitchen with {} rooms.", n),
        d => println!("{:?}", d),
    }

    if let Lounge(n, s) = l {
        println!("You are in the {} lounge with {} chairs.", s, n);
    }

    /*
    let v = match t {
        Kitchen(n) => n,
        _ => 0,
    };
    */

    /*
    if let Kitchen(n) = t {
        println!("Its a kitchen with {} cupboards.", n);
    }
    */
}

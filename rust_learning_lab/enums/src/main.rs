/*
enums are used when we you can enumerate all possible
variants.

enum values can only be one of its variants.
*/

#[derive(Debug)]
enum IpAddrVersions {
    V4(u8, u8, u8, u8),
    V6(u16, u16, u16, u16, u16, u16, u16, u16),
}

#[derive(Debug)]
enum UsState {
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let addrv4 = IpAddrVersions::V4(127, 0, 0, 1);
    let addrv6 = IpAddrVersions::V6(0, 0, 0, 0, 0, 0, 0, 1);

    println!("Localhost v4 Addr: {:?}", addrv4);
    println!("Localhost v6 Addr: {:?}", addrv6);

    // match example
    let money = Coin::Quarter(UsState::Alaska);
    println!("Wow you have {} cents.", value_in_cents(money));


    // plus_one example
    let four = Some(4);
    let five = plus_one(four);
    let none = plus_one(None);

    println!("five has a value of {:?}.", five);
    println!("none has a value of {:?}.", none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?} found.", state);   
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
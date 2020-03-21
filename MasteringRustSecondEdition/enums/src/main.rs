/*
Enums - used for modeling something of different kinds. Types are
called 'variants' Variants can be defined with or without data.
The data can be any primitive type, struct, tuple struct, or even
an enum. Enums can also be created on the stack so they need to
have a predetermined size. 
*/

#[allow(dead_code)]
#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W
}

#[allow(dead_code)]
#[derive(Debug)]
enum PlayerAction {
    Move {
        direction: Direction,
        speed: u8
    },
    Wait,
    Attack(Direction)
}

fn main() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };

    match simulated_player_action {
        PlayerAction::Wait => println!("Player wants to wait"),
        PlayerAction::Move {direction, speed} => {
            println!("Player wants to move in direction {:?} with speed {}", direction, speed)
        }
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
    };
}

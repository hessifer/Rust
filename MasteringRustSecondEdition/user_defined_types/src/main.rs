// user defined types - structs, enums, and unions -> CamelCase


/*
three forms of structs - unit, tuple, c-like (named)

unit struct - used to model entities with no data or state associated. 
Also error types where the name itself is sufficient. Also to represent
states in a state machine implementation.

tuple struct - has associated data, but the fields are not named and are
referred to by position in the definition. Tuple struct is ideal when
you have data with less than four or five attributes.

c-like (named) struct - has associated data and the fields are named. Used for
data types that have more than four fields / attributes. Order does not matter.

NOTE: the size_of a struct is simply the sum of its fields, along with any data alignment padding.
*/

// unit struct
#[derive(Debug)]
struct Dummy;

// tuple struct
struct Color(u8, u8, u8);

// c-like (named) struct
struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += score;
    println!("Updated player stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}

fn main() {
    let value = Dummy; // no value, just name for unit structs
    println!("{:?}", value);

    let white = Color(255,255,255);

    // You can pull them out of the index
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}", blue);

    println!();
    let orange = Color(255, 165, 0);

    // You can also destructure the fields directly
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);

    // Can also ignore fields while destructuring
    let Color(r, _, b) = orange;
    println!("R: {}, B: {} (orange)", r, b);

    println!();    
    // c-like (named) struct usage
    let name = "Alice".to_string();
    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129
    };
    bump_player_score(player, 120);
}
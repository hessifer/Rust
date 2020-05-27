enum HockeyPosition {
    Center,
    LeftWing,
    RightWing,
    Goalie,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8, 
}

impl HockeyPlayer {
    // associated function (no self keyword) to create instance of type HockeyPlayer 
    fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer {
            name: name,
            number: number,
            position: position,
            goals_ytd: 0,
        }
    }

    // borrow mutable reference to self and do not transfer ownership
    fn shoot_puck(&mut self, seconds_remaining: u16) {
        if seconds_remaining < 300 {
            match self.position {
                HockeyPosition::Center => println!("Goal!"),
                _ => println!("Miss!"),
            }
        } else {
            println!("Goal!");
        }
    }
}

fn main() {
    // use associated function to create instance of HockeyPlayer
    let mut player = HockeyPlayer::new(
        String::from("Charles Hessifer"),
        5,
        HockeyPosition::Goalie,
    ); 

    // modify from default of 0
    player.goals_ytd = 25;
    print!(
        "The player wearing number {} is ",
        player.number
    );

    println!(
        "{} and has scored {} goals this season",
        player.name,
        player.goals_ytd,
    );

    player.shoot_puck(420);
    player.shoot_puck(120);
    player.shoot_puck(12);
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

enum HockeyPosition {
    RightWing,
    LeftWing,
    Goalie,
    Center,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

fn tell_time(clock: Clock) {
    match clock {
        // destructure value(s) contained in our clock
        Clock::Sundial(hours) => {
            println!("It is about {} O'clock.", hours);
        },
        Clock::Digital(hours, minutes) => {
            println!("It is {} minutes past {}.", minutes, hours);
        },
        Clock::Analog(hours, minutes, seconds) => {
            println!("It is {} minutes and {} seconds past {}.", minutes, seconds, hours);
        },
    }
}

fn main() {
    tell_time(Clock::Analog(9, 25, 45));

    let player = HockeyPlayer {
        name: String::from("Charles Hessifer"),
        number: 5,
        position: HockeyPosition::LeftWing,
        goals_ytd: 22,
    };

    println!("{} wears number {} and has {} goals so far.", player.name, player.number, player.goals_ytd);
}
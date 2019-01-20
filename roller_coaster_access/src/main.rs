/*
    Program: roller_coaster_access
    Author:  unknown
    Date:    Jan 19, 2019
    Purpose: This program will take input from the user (Name, Age, Choice) to determine
             which roller coaster they have access to and if they want to ride it. If they
             are allowed to ride the roller coaster and they desire to we will provide them
             with a boarding number.
*/
use std::io;


fn main() {
    let mut first_name = String::new();
    let mut age = String::new();
    let mut choice = String::new();
    let mut speed_demon = false;

    println!("Please enter your first name and age");
    io::stdin().read_line(&mut first_name).expect("ERROR: Could not parse first name.");
    io::stdin().read_line(&mut age).expect("ERROR: Could not parse age.");
    let age:i8 = age.trim().parse().expect("ERROR: could not parse age to 8 bit integer.");

    println!("Would you like to ride the [fast] or [slow] roller coaster? ");
    io::stdin().read_line(&mut choice).expect("ERROR: Could not parse choice.");
    choice = choice.trim().to_string(); // converts ref to string to string

    if choice == "fast" {
        if age > 12 {
            println!("Nice, buckle up and enjoy the fast roller coaster!");
            speed_demon = true;
        } else {
            println!("Good choice, unfortunately you need to be 13 years of age or older.");
            println!("Would you like to ride the slow roller coaster instead? [yes/no]: ");
            choice.clear();
            io::stdin().read_line(&mut choice).expect("ERROR: Could not parse choice.");
            if choice.trim().to_string() == "yes" {
                println!("Enjoy the ride.");
            } else {
                println!("Have a nice day and enjoy the park!");
            }
        }
    } else {
        println!("Enjoy the ride.");
    }

    let took_fast_roller_coaster = if speed_demon {
        true
    } else {
        false
    };

    if took_fast_roller_coaster {
        println!("Please stop by the photo booth to view/purchase your picture.");
    }
}

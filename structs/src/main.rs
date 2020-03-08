struct User {
    username: String,
    email: String,
    active: bool,
    is_admin: bool,
}

struct Car {
    year: i32,
    make: String,
    model: String,
    has_four_doors: bool,
    number_of_tires: i32,
}

fn main() {
    let user1 = User {
        email: String::from("charles@email.com"),
        username: String::from("charles"),
        active: true,
        is_admin: false,
    };

    if user1.active {
        println!("Username: {}\n\tEmail: {}\n\tAdmin: {}", user1.username, user1.email, user1.is_admin);
    }

    // Create Car Instance
    let mycar = Car {
        year: 2019,
        make: String::from("Jeep"),
        model: String::from("Wrangler"),
        has_four_doors: true,
        number_of_tires: 4,
    };

    println!("\nMy Car Details:\n\tYeah: {}\n\tMake: {}\n\tModel: {}\n\tNumber of Tires: {}", mycar.year, mycar.make, mycar.model, mycar.number_of_tires);
    if mycar.has_four_doors {
        println!("\nWow, nice Jeep!");
    }
}

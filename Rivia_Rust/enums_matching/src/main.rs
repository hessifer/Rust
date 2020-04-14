#[derive(Debug)]
#[allow(dead_code)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Desert {
    Cake,
    IceCream(Size),
    Candy,
    MilkShake(Size),
    Pie,
}

#[derive(Debug)]
struct Customer {
    name: String,
    ticket_number: i32,
    order_time: String,
    is_member: bool,
    order: Desert,
}

fn main() {
    let customer_01 = Customer {
        name: String::from("Charles"),
        ticket_number: 465,
        order_time: String::from("16:43"),
        is_member: false,
        order: Desert::MilkShake(Size::Medium),
    };

    if let Desert::MilkShake(size) = customer_01.order {
        println!("{}, I hope you enjoy your {:?} Milk Shake!", customer_01.name, size);
    } else {
        println!("{}, enjoy your meal.", customer_01.name);
    }

    if customer_01.is_member {
        println!("Thanks for being a value savings member!")
    } else {
        println!("Would you like to become a member and save 10% today?");
    }

    print!("Order Details\nTicket Number: {}\nOrder Time: {}\n", customer_01.ticket_number,
            customer_01.order_time);
}

fn main() {
    let msg = String::from("Sup!");
    let count = 5;

    display_message(msg, count);
    array_loop();
}

fn display_message(msg: String, count: isize) {
    let mut iterations = count;
    // let times = count;

    // while loop example
    while iterations > 0 {
        println!("{}", msg);
        iterations -= 1;
    }

    // for loop example
    for i in { 0..10 } {
        println!("{}\t\t{}", i, msg.to_uppercase());
    }

    // loop example
    /*loop {
        *times -= 1;
        println!("{}", msg);

        if *times <= 0 {  // need to add condition check to avoid infinite loop
            break;
        }
    }*/
}

fn array_loop() {
    let mut my_vector = Vec::new();

    my_vector.push(9);
    my_vector.push(8);
    my_vector.push(55);
    my_vector.push(32);

    for i in &my_vector {
        println!("{}", i);
    }

    let new_vector = vec![100, 300, 500]; // shortcut way to declare vector

    for (index, value) in new_vector.iter().enumerate() {
        println!("new_vector[{:?}]:\t{:?}", index, value);
    }

    for j in &my_vector {
        if j % 2 == 0 {
            println!("{} is even.", j);
        }
    }

    for element in new_vector.iter() {
        println!("value is: {}", element);
    }

    // print the array backwards
    for number in (0..new_vector.len()).rev() {
        println!("index {}: {:?}", number, new_vector[number]);
    }
}

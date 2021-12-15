/*
* 1. Ask the user to enter 6 colors
* 2. Add those colors to a vector
* 3. Ask user for a pattern
* 4. Display all colors with specified pattern, or indicate no matches
* 5. Display all colors with an even number length
*/
use std::io::Write;

fn main() {
    let colors = collect_colors();
    let pattern = get_pattern();
    let mut numbered_color: Vec<Vec<(usize, String)>> = vec![];

    for color in &colors {
        if color.contains(&pattern) {
            println!("Color: {}", color);
        }
    }

    // add capacity for numbered_color
    for _ in colors.iter() {
        numbered_color.push(Vec::with_capacity(2));
    }

    println!("The following colors have an even numbered length.");
    for color in &colors {
        if color.len() % 2 == 0 {
            print!("{} ", color);
        }
    }

    println!();

    // add data for numbered_color vectors
    for (i, color) in colors.iter().enumerate() {
        numbered_color[i].push((i, String::from(color)));
    }

    // display data for number_color
    for (i, data) in numbered_color.iter().enumerate() {
        for (j, _) in data.iter().enumerate() { // not necessary if we know we have 1 element
            println!("{} - Element {} -> {}", i, data[j].0, data[j].1);
        }
    }
}

fn collect_colors() -> Vec<String> {
    // create vector of strings with a capacity of 6
    let mut colors: Vec<String> = Vec::with_capacity(6);

    while colors.len() != 6 {
        // get 6 colors from user and return them
        let mut color = String::new();

        print!("Please enter a color: ");
        std::io::stdout().flush().expect("Unable to flush stdout.");
        match std::io::stdin().read_line(&mut color) {
            Ok(_) => {
                let result = color.trim().to_lowercase();

                if colors.contains(&result) {
                    continue;
                } else {
                    colors.push(color.trim().to_lowercase());
                }
            }
            Err(_) => continue,
        }
    }
    colors
}

fn get_pattern() -> String {
    let mut pattern = String::new();

    print!("Please enter a search pattern: ");
    std::io::stdout().flush().expect("Unable to flush stdout.");
    match std::io::stdin().read_line(&mut pattern) {
        Ok(n) => println!("{} bytes read successfully.", n),
        Err(e) => println!("ERROR: {}", e),
    }
    String::from(pattern.trim())
}

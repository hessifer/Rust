// working in here for enums instead of data_structures
enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8), // tuple type definition
    CmykColor { // struct type definition (helpful for clarity in certain cases)
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8
    }
}

pub fn enums() {
    // let c: Color = Color::RgbColor(0,0,0);
    let c: Color = Color::CmykColor {cyan: 8, magenta: 4, yellow: 2, black: 0};

    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RgbColor(0, 0, 0) |
            Color::CmykColor {cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        // Color::CmykColor {cyan: c, magenta: m, yellow: y, black: k} => println!("cmyk({}{}{}{})", c, m, y, k)
        _ => () // our catch all that does nothing :`)
    }
}
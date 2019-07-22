use std::env::args;

fn main() {
    let args_size = args().count();

    if args_size > 0 {
        for arg in args()  {
            if let Some(c) = arg.chars().next() {
                match c {
                    'w' | 'W' => println!("Hello {}!", arg),
                    _ => {}
                }
            }
        }
    }
}

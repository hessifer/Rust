fn main() {
    let mut v = Vec::new(); // vector already included in prelude (no need to import)

    v.push('a');
    v.push('b');
    v.push('c');

    for item in v {
        print!("{}", item);
    }
}

fn change_string(s: &mut String) {
    s.push_str(" Rust");
}

fn main() {
    let mut s = String::from("Hello");
    change_string(&mut s);
    dbg!(s);
}

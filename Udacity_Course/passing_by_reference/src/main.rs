fn concat(s1: &mut String, s2: &String) {
    for ch in s2.chars() {
        s1.push(ch);
    }
}

fn main() {
    let mut s1 = String::from("hello");
    let s2 = String::from(" world");
    concat(&mut s1, &s2);
    dbg!(s1);
}

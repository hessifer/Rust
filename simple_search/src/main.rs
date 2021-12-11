fn main() {
    let search_term = "linux";
    let quote = "\
    Linux, careful you might learn something and
    all work and no play
    git bit by byte linux is hype";

    for (i, line) in quote.to_lowercase().lines().enumerate() {
        if line.contains(search_term) {
            println!("{}: {}", i + 1, line);
        }
    }
}

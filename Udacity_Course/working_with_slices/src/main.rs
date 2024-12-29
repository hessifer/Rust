// Slice is a reference to a contiguous sequence of elements in a collection rather than the whole collection.
fn trim_tweet(tweet: &str) -> &str {
    // deref coercion: &String -> &str
    let end_of_tweet = tweet.len();
    &tweet[5..end_of_tweet]
}
fn main() {
    let tweet = String::from("This is a tweet message");
    let trimmed_tweet = trim_tweet(&tweet); // This is a slice of the tweet string. (no need for 0, since we start at the beginning of the string)
    dbg!(trimmed_tweet);
}

/* String Types
* String: A growable, heap-allocated data structure (UTF-8 encoded)
* &str: A slice of a string, a reference to some UTF-8 encoded string data stored elsewhere (static, heap, or stack memory)
* Handle behind a reference (&str) b/c length of the sequence is not known at compile time.
*/

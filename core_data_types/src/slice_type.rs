// another data type that does not have ownership is the slice
// 'slice' lets you reference a contiguous sequence of elements in a collection rather than the
// entire collection.

pub fn first_word(s: &str) -> &str { // return literal str of first word, using ref since don't need ownership
   let bytes = s.as_bytes(); // how many bytes does s consume (may want to consider UTF8)

    for (i, &item) in bytes.iter().enumerate() { // returns an index and reference time
        if item == b' ' { // byte literal syntax
            return &s[0..i]; // if we found a space, return from byte 0 to index[i] or first word
        }
    }
    &s[..] // no spaces found return entire slice (whole word)
}
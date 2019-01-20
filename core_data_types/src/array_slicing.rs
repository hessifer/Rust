// just a quick example of array slicing, more to come later
pub fn a_bit_of_array(a: &[i32]) -> &[i32] {
    // return index 1 - 5 of array
    &a[1..5]
}

pub fn calc_fib_term(term: u32) -> u32 {
    if term == 0 {
        0
    } else if term == 1 {
        1
    } else {
        calc_fib_term(term - 1) + calc_fib_term(term - 2)
    }
}
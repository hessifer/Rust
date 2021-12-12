fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
    Every face, every shop, bedroom window, public-house,
    and dark square is a picture feverishly turned--in search
    of what? It is the same with books. What do you seek
    through millions of pages?";

    // hold line numbers where matches are found
    let mut tags: Vec<usize> = vec![];

    // contains a vector per match to hold context lines
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    // iterate through each line and record line numbers for matches
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            /*
             * Vec::with_capacity(n) reserves space for n items. This minimizes
             * the number of times memory will need to be allocated from the OS.
             */
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    // if we did not find any matches exit
    if tags.is_empty() {
        return;
    }

    // for each tag at every line see if we are near a match. If so, add line
    // to the respective Vec<T> within ctx
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            /*
             * saturating_sub() is subtraction that returns 0 on integer
             * underflow rather than crashing the program. CPUs do not like
             * attempting to send usize below zero.
             */
            let lower_bond = tag.saturating_sub(ctx_lines);
            let upper_bond = tag + ctx_lines;

            if (i >= lower_bond) && (i <= upper_bond) {
                // copy line into a new String and store locally for
                // each match
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        // ref line informs the compiler that we want to borrow this
        // value rather than move it.
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

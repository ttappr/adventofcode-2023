[LANGUAGE: Rust] 🦀

I decided to approach this challenge as a Longest Palindromic Subsequence type problem. In the process of finding an example of the DP LPS algorithm, I happened on the [Manacher's algorithm](https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm) for finding LPS's. I modified this to produce the list of all palindromic sequences instead of just the maximum. In the linked source code, this is the `palindromes()` function.

`palindromes()` returns a vector of tuples representing the palindromes in a sequence. Each tuple consist of the radius of a palindrome and the index of its center. If there is an axis of symmetry running vertically through a matrix with `n` rows, after running `palindromes()` on each row, we should find `n` palindromes (one in each row) with the same center and radius to the edge of the matrix. If not, rotate the matrix and try again.

In part two, we're looking for `n - 1` palindromes with a common center and radius after processing each row of a matrix.

[Full Source](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
        let mut counter  = HashMap::new();
        let     matrices = get_matrices(path)?;
        let mut total    = 0;
        for mut m in matrices {
            let mut done = false;
            counter.clear();
            for r in &m {
                for p in palindromes(r) {
                    *counter.entry(p).or_insert(0) += 1;
                }}
            for (p, &c) in &counter {
                if c == m.len() - 1 {
                    total += p.1;
                    done = true;
                    break;
                }}
            if !done {
                m = rot90_counter_clockwise(&m);
                counter.clear();
                for r in &m {
                    for p in palindromes(r) {
                        *counter.entry(p).or_insert(0) += 1;
                    }}
                for (p, &c) in &counter {
                    if c == m.len() - 1 {
                        total += p.1 * 100;
                        break;
                    }}}}
        println!("Part 2 Total: {}", total);
        Ok(total)
    }
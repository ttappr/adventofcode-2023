[LANGUAGE: Rust] 🦀

Part 1 wasn't difficult. I used [UnionFind](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) to get the size of the group of points with no connection to the edges, and had it completed pretty quickly. Then I attempted the same approach with Part 2, which didn't work due to the size of the disjoint set array that would be needed. 

So, I thought I'd try another idea where I organize all the horizontal lines in an ordered set then iterate over them filling the space between lines. Simple idea in concept, but it took me down a rabbit hole of off-by-one's and endless debug until I got it right. Ugh! Perfecting this approach was hard won knowledge.

Below is the section of code that determines how many column tiles are filled in the space between horizontal lines. Most of the rest of the code deals with parsing the input data into grid lines.

The solution runs pretty fast on part 2's data, completing in a fraction of a second.

[Full Code](https://topaz.github.io/paste/#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)

    // Iterate over the horizontal line end points.
    while let Some((mut p1, mut p2)) = iter.next() {
        let r1 = p1.0;
        loop { // on all horizontals in the same row.
            let (_, c1) = p1;
            let (_, c2) = p2;

            for c in c1..=c2 {
                match col_state![c] {
                    Open(r2) => {
                        if c == c1 || c == c2 {
                            col_state![c] = End(r1);
                        } else {
                            col_state![c] = Closed(r1);
                        }
                        capacity += r1 - r2 - 1;
                    },
                    End(r2) => {
                        if c == c1 || c == c2 {
                            col_state![c] = End(r1);
                            if verticals.remove(&((r2, c), (r1, c))) 
                                || c == c1 && col_state![c + 1].is_open() 
                                || c == c2 && col_state![c - 1].is_closed() {
                                capacity += r1 - r2 - 1;
                            }
                        } else if col_state![c - 1].is_open()
                            || col_state![c + 1].is_closed() {
                            col_state![c] = Open(r1);
                        } else {
                            col_state![c] = Closed(r1);
                            capacity += r1 - r2 - 1;
                        }
                    },
                    Closed(_) => {
                        if c == c1 || c == c2 {
                            col_state![c] = End(r1);
                        } else {
                            col_state![c] = Open(r1);
                        }
                    },
                }
                capacity += 1;
            }
            if iter.peek().map_or(true, |(p1, _)| p1.0 != r1) { break; }
            (p1, p2) = iter.next().unwrap();
        }
    }
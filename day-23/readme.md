[Language: Rust]  🦀

I took quite a few wrong turns on this one. Eventually, I implemented something close to [this example](https://www.geeksforgeeks.org/find-longest-path-directed-acyclic-graph/) for Part 1.

For Part two, I implemented a run-of-the-mill DFS traversal function to explore all paths. It wouldn't complete until I added an [edge contraction](https://en.wikipedia.org/wiki/Edge_contraction) feature to my `Vertex` class.

All my functions are iterative - no recursion. Hopefully, someone may find my non-recursive DFS example of some use. Or the non-recursive topological sort.

[Full Code](https://topaz.github.io/paste/#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)

From the `Vertex` class (see Full Source above). This was the magic that got DFS working for Part 2.

    fn collapse_edges(&mut self, graph: &GraphP2) {
        for adj_i in 0..self.degree() {
            let mut curr   = &*self;
            let mut weight = 1;
            let mut adj_j  = adj_i;
            while let Some(adj_v) = graph.get(&curr.edge(adj_j)) {
                if adj_v.degree() == 2 {
                    if adj_v.edge(0) == curr.locus {
                        adj_j = 1;
                    } else {
                        adj_j = 0;
                    } 
                    weight += 1;
                    curr = adj_v;
                } else {
                    curr = adj_v;
                    break;
                }
            }
            self.edges[adj_i] = (curr.locus.0, curr.locus.1, weight);
        }
    }
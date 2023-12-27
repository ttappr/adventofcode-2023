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
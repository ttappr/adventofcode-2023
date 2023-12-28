[LANGUAGE: Rust]  🦀

IDK if there's a part 2 to this - I have a few stars to catch up on and wondering if it unlocks another puzzle or simply ends the quest. I'll find out eventually. Anyway, I'm posting Part 1 early since I think someone may find my Part 1 solution useful/interesting. I found a useful crate (at least it was useful for the first half of this problem) that supports graph operations like finding the minimum cut (as I worked out below). The crate is [rustworkx_core](https://docs.rs/rustworkx-core/latest/rustworkx_core/index.html). I had implemented Karger's minimum cut algorithm for Part 1, but got tired of debugging =). I'll probably come back to it though. Anyway, here's some code that shows how to leverage the rustworkx library.

Judging from my experience today hacking at the [Karger algorithm](https://en.wikipedia.org/wiki/Karger%27s_algorithm), I believe the [Stoer–Wagner algorithm](https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm) may be easier to implement, is probably faster, and has what look like good example implementations on the Wikipedia link I provided.

[Full Code Part 1](https://topaz.github.io/paste/#XQAAAQDzCgAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1VrqiATAYXgXakDYg7AoIwE0zcXY/d57S2rM3jmt9tZm+Dy0Noj+ayR7xhcWiBu7UwKbVJjzpN9GUF/sK6VafDCQo83PqX5oecZ/nviYy+T93C4dCB4H19Vcl/jiAfgiLAQYwRE0/mZtd7nKi73hP6NqyX+uia2p87Kg5TuwzQ/vfgLRT/q+V76usiAhkKgFsSGWJXJ5dEt/6tfwYS1egpzqIubTaDBwWu5gnpQpFCbH25/G/AER795aTVd0gL2+4V9PHe4TSTsJvIVkg37ygakvF3wKIcwSvD+OC1OhKRdRV1Kz+bGT0tiyl1pTvW8QTWJDjJw5YOOotTWRmA6MhQFhli76KscgDnR6s7cEL+MmJUm1pkCO+TWYuYpauiPMrYLIXqZLB6mEO3us1FUyOSi4bMxZbICsA/rF9aGlVWgKare+rbvvNa4U63rSikR+LoQzgYuaOJi78JpftNLxwBDNvJA2malfnvPG/mH/qKxdVxUyb+tAMBI6tTd3f5huvhYDdGMMo4lu28YirAfFbSc6076K4R6mu14VLz0MbPkmBiXDg4B32uF2NdExkvZZgsLsOBGDXNK+FysSF006p3kUoiwJwSg8kS+1EAMy37MYXTmZCqPzdP0jPKyVl3oyMzIHXpTAr7JiMTKkp2UoXcU/T8So7OCrC+4P56TeVz953asDjZYxvS88lryVN2QQr88qaAOsaf58wrXGp7+s3HbFw4X5W8ZKCpqgj/H+p3rRObbeAUmTr4S+dhdvYa8OHkGF9zP7c/32rM5P4aS6+6qK3UOqrVdarC4lsS0y1Cy2sJHh+XTzSrHVMvXN9xxiJ20YJ4I/00jrHn+oRhyZ7q7WGNLABiX0GghfvRO+tt/Vpv5Rs0wELT6MxYPZqcvRp9g3ADp1s8z8EnYFckuXJ0hds4n4Eq0FLmnfTxrlhZEJ5AcTflX8l/EvuTsUHL4HYVFAvQPd54OGz/XCCXKB10joKDAcPzA4HimjUv96Lsx/eE/f7CsvWC6SFyR5uRiQLh7R3OZWZiug8c79ZQMJOKxkEgdnQVNQNvb14Q/76n7Gff+5Mn9wrjJ7l8OiK/ZFmDe0MXhMcqZJpFDAiPoRTuUeCPdM2w2XfaiLjGIp8D+okhNakht1XJQtC9XGSO3YhHz3+eWjb7xtynvAntgobx4PYxFaHg84L/upcYTUB5XlgcmmAlXy6+4tjbpMfSWkHoowDeZvLBQLOTTa1NbhEJKBhRxdD+lmp5vj54yPtP8Kwiuk2OBmDkxeOPyQ4tRdYmW5q4SuI3jO/MiloLOPZLUfT5hwfD8v3DDp66xCigkaVyPYphnXjnVtD5ePcx9n6BqsaqqFG1M0KGIjYHrCRMONlT3XZnwZTNrrCyF0I/9dj8GWRWSxFpdEzgaqASXIS52lK3+wrY7bcdl3VamHVHkGFqfCNx3TwyMrEEDX0Jy4bHIPvFJfsce+W69Aq9YAabItNRi7+JHxLHJyXnbDGi/WsN55feLHHLrjgaIHEV3rHmqFaknsP0PTXj4LrcmM2Azbi4Or+aH+xbdKXdQphclP/74hht)

    fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
        let     edges = parse_input(path)?;
        let mut graph = UnGraph::new_undirected();

        let verts = edges.iter().flatten().map(|s| s.as_str())
                                .collect::<HashSet<_>>();
        let nodes = verts.iter().map(|&s| (s, graph.add_node(s)))
                                .collect::<HashMap<_, _>>();

        for adjacent in &edges {
            let v1 = adjacent[0].as_str();

            for v2 in adjacent[1..].iter().map(|s| s.as_str()) {

                graph.add_edge(nodes[v1], nodes[v2], 1);
            }
        }
        let min_cut: RwxResult<_> = stoer_wagner_min_cut(&graph, |_| Ok(1));

        if let Ok(Some((_, cut))) = &min_cut {
            let product = (verts.len() - cut.len()) * cut.len();

            println!("Part 1 Product of Subgraph Sizes: {}", product);
        } else {
            println!("Unable to find min cut!");
        }
        Ok(0)
    }
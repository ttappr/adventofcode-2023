//! Advent of Code 2023 Day 23 - A Long Walk
//! 
//! Part 1: Find the longest hike you can take through the hiking trails listed 
//!         on your map. How many steps long is the longest hike?
//! Part 2: Find the longest hike you can take through the surprisingly dry 
//!         hiking trails listed on your map. How many steps long is the longest 
//!         hike?

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 23 Advent of Code\n"); 

    let start = Instant::now();

    for input in ["./data/sample1.txt", 
                  //"./data/input1.txt"
                  ] {
        part_1(input)?;
        part_2(input)?;
    }

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Find the longest hike you can take through the hiking trails listed
///         on your map. How many steps long is the longest hike?
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let map    = lines.map(|l| Ok(l?.chars().collect())).collect::<RawMap>()?;
    let (m, n) = (map.len(), map[0].len());
    let start  = (0usize, 1usize);
    let end    = (m - 1, n - 2);

    let mut vertices = Vec::new();
    let mut adjacent = HashMap::new();

    macro_rules! map { ($p: expr) => { (map[$p.0][$p.1]) } }
    
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if map[i][j] != '#' {
                vertices.push((i, j));
            }
            let test = [(i.saturating_sub(1), j), (i, j.saturating_sub(1)), 
                        ((i + 1).min(m - 1),  j), (i, (j + 1).min(n - 1))];
            let mut adj2 = Vec::new();
            if !['#', 'v'].contains(&map!(test[0])) { adj2.push(test[0]); }
            if !['#', '>'].contains(&map!(test[1])) { adj2.push(test[1]); }
            if !['#', '^'].contains(&map!(test[2])) { adj2.push(test[2]); }
            if !['#', '<'].contains(&map!(test[3])) { adj2.push(test[3]); }
            adjacent.insert((i, j), adj2);
        }
    }
    let dists = longest_path(&vertices, &adjacent, start);
    let steps = *dists.get(&end).unwrap_or(&0) - 2;

    println!("Part 1 Total steps...: {}", steps);
    Ok(steps)
}

/// Part 2: Find the longest hike you can take through the surprisingly dry 
///         hiking trails listed on your map. How many steps long is the longest
///         hike?
/// 
fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let map    = lines.map(|l| Ok(l?.chars().collect())).collect::<RawMap>()?;
    let (m, n) = (map.len(), map[0].len());
    let start  = (0usize, 1usize);
    let end    = (m - 1, n - 2);

    let mut graph = GraphP2::new();

    macro_rules! map { ($p: expr) => { (map[$p.0][$p.1]) } }
    
    // Create the adjacency map. 
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if map[i][j] == '#' { 
                continue; 
            }
            let test = [(i.saturating_sub(1), j), (i, j.saturating_sub(1)), 
                        ((i + 1).min(m - 1),  j), (i, (j + 1).min(n - 1))];

            let mut adj = Vec::with_capacity(4);

            for p in &test {
                if map!(*p) != '#' { adj.push(*p); }
            }
            adj.retain(|p| *p != (i, j));
            graph.insert((i, j), Vertex::new((i, j), adj));
        }
    }
    // Collapse the edges of each Vertex that has more than two edges.
    for locus in graph.keys().copied().collect::<Vec<_>>() {
        if graph[&locus].degree() > 2 {
            let mut v = graph[&locus].clone();
            v.collapse_edges(&graph);
            graph.insert(locus, v);
        }
    }
    // Basic DFS traversal to find longest path to end.
    let steps = dfs(start, end, &graph);

    println!("Part 2 Total steps...: {}", steps);
    Ok(steps)
}

/// Some gratuitious type aliases to make the code more readable (or not...).
/// 
type RawMap  = Result<Vec<Vec<char>>, Box<dyn Error>>;
type GraphP1 = HashMap<Locus, Vec<Locus>>;
type GraphP2 = HashMap<Locus, Vertex>;
type DistMap = HashMap<Locus, usize>;
type Locus   = (usize, usize);
type WEdge   = (usize, usize, usize); // Weighted edge.

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Vertex {
    locus : Locus,
    edges : Vec<WEdge>,
}
impl Vertex {
    fn new(pos: Locus, adj: Vec<Locus>) -> Self {
        Self { 
            locus: pos, 
            edges: adj.into_iter().map(|p| (p.0, p.1, 1)).collect() 
        }
    }
    /// Updates each edge with a direct edge to the closest junction Vertex that
    /// has more than two edges.
    /// 
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
    /// The degree of the vertex (how many edges it has).
    /// 
    fn degree(&self) -> usize {
        self.edges.len()
    }
    /// Returns a specific edge of the vertex at the given index.
    /// 
    fn edge(&self, i: usize) -> Locus {
        (self.edges[i].0, self.edges[i].1)
    }
    /// Returns the weight of a specific edge of the vertex at the given index.
    /// 
    fn edge_weight(&self, i: usize) -> usize {
        self.edges[i].2
    }
}

/// Basic DFS traversal to find longest path to end. The implementation is
/// iterative to avoid stack overflow.
/// 
fn dfs(start: Locus, end: Locus, graph: &GraphP2) -> usize {
    enum Stage<T> { 
        Enter(T, usize), 
        Loop(T, usize, usize),
        Finish(T) 
    } use Stage::*;

    let mut seen   = HashSet::new();
    let mut lstack = vec![Enter(start, 0)];
    let mut d_max  = 0;

    while let Some(stage) = lstack.pop() {
        match stage {
            Enter(v, depth) => {
                if v == end {
                    d_max = d_max.max(depth);
                } else {
                    seen.insert(v);
                    lstack.push(Loop(v, 0, depth));
                }
            },
            Loop(v, i, depth) => {
                if i == 0 { lstack.push(Finish(v)); }
                if i < graph[&v].degree() {
                    let v2 = graph[&v].edge(i);
                    let wt = graph[&v].edge_weight(i);
                    lstack.push(Loop(v, i + 1, depth));
                    if !seen.contains(&v2) {
                        lstack.push(Enter(v2, depth + wt));
                    }
                }
            },
            Finish(v) => {
                seen.remove(&v);
            },
        }
    }
    d_max
}

/// Finds the longest path from a given start vertex to all other vertices in
/// the directed acyclic graph. The implementation is iterative to avoid stack 
/// overflow.
/// 
fn longest_path(verts: &[Locus], graph: &GraphP1, start: Locus) -> DistMap {
    let mut dists = HashMap::new();
    let mut seen  = HashSet::<Locus>::new();
    let mut stack = Vec::new();

    for v in verts {
        if !seen.contains(v) {
            topological_sort(*v, &mut stack, &mut seen, graph);
        }
    }
    dists.insert(start, 0);

    while let Some(u) = stack.pop() {
        if dists.contains_key(&u) {
            for v in &graph[&u] {
                let d1 = *dists.get( v).unwrap_or(&usize::MIN);
                let d2 = *dists.get(&u).unwrap_or(&usize::MIN) + 1;
                if d1 < d2 {
                    dists.insert(*v, d2);
                }
            }
        }
    }
    dists
}

/// Topological sort of a directed acyclic graph. The implementation is
/// iterative to avoid stack overflow.
/// 
fn topological_sort(vertex : Locus,
                    stack  : &mut Vec<Locus>,
                    seen   : &mut HashSet<Locus>,
                    graph  : &GraphP1)
{
    enum Stage<T> { Enter(T), Loop(T, usize), Finish(T) } use Stage::*;

    let mut lstack = vec![Enter(vertex)];

    while let Some(stage) = lstack.pop() {
        match stage {
            Enter(v) => {
                seen.insert(v);
                lstack.push(Loop(v, 0));
            },
            Loop(v, i) => {
                if i == 0 { lstack.push(Finish(v)); }
                if let Some(v2) = graph[&v].get(i) {
                    lstack.push(Loop(v, i + 1));
                    if seen.insert(*v2) {
                        lstack.push(Enter(*v2));
                    }
                }
            },
            Finish(v) => {
                stack.push(v);
            },
        }
    }
}

//! Advent of Code 2023 Day 17 - Clumsy Crucible
//! 
//! Part 1: Direct the crucible from the lava pool to the machine parts factory 
//!         with the least heat loss.
//! Part 2: Direct the larger crucible from the lava pool to the machine parts 
//!         factory with the least heat loss.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 16 Advent of Code\n"); 

    let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Direct the crucible from the lava pool to the machine parts factory 
///         with the least heat loss.
/// 
fn part_1(path: &str) -> Result<i32, Box<dyn Error>> {
    let grid = get_grid(path)?;

    let c = dijkstra(&grid, Vertex::new(0, 0), 0, 3);

    println!("Part 1): {}", c);
    
    Ok(c)
}

/// Part 2: Direct the larger crucible from the lava pool to the machine parts 
///         factory with the least heat loss.
/// 
fn part_2(path: &str) -> Result<i32, Box<dyn Error>> {
    let grid = get_grid(path)?;

    let c = dijkstra(&grid, Vertex::new(0, 0), 4, 10);

    println!("Part 2): {}", c);
    
    Ok(c)
}

/// Implements the Dijkstra shortest path algorithm.
/// 
fn dijkstra(g     : &[Vec<u8>], 
            start : Vertex, 
            min   : u8, 
            max   : u8) 
    -> i32
{
    let (m, n) = (g.len(), g[0].len());
    let mut heap = BinaryHeap::new();
    let mut cost = HashMap::new();
    let mut seen = HashSet::new();
    let mut best = i32::MAX;

    cost.insert(start, 0);
    heap.push(Reverse((0, start)));

    while let Some(Reverse((_, v))) = heap.pop() {
        seen.insert(v);

        for adj in v.adjacent(m, n, min, max) {
            if !seen.contains(&adj) {
                let f = |c| c + (g[adj.i][adj.j] - b'0') as i32;
                let c = cost.get(&v).map_or(i32::MAX, f);

                if c < *cost.get(&adj).unwrap_or(&i32::MAX) {
                    if adj.pos() == (m - 1, n - 1) { 
                        best = best.min(c); 
                    }
                    cost.insert(adj, c);
                    heap.push(Reverse((c, adj)));
                }
            }
        }
    }
    best
}

/// Vertex struct for the graph. Holds the row and column coordinates of the
/// vertex. Additionally holds the direction it's going and the number of
/// consecutive steps in that direction. I guess you could call it a vector
/// instead of a vertex.
/// 
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    i     : usize,
    j     : usize,
    dir   : u8,
    count : u8,
}
impl Vertex {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j, dir: 0, count: 0 }
    }
    fn new2(parent: Vertex, dir: u8) -> Self {
        let mut child = parent;
        if dir == child.dir { child.count += 1; } 
        else                { child.dir    = dir; 
                              child.count  = 1; }
        match dir {
            b'N' => child.i -= 1,
            b'S' => child.i += 1,
            b'W' => child.j -= 1,
            b'E' => child.j += 1,
            _ => unreachable!(),
        }
        child
    }
    fn pos(&self) -> (usize, usize) {
        (self.i, self.j)
    }
    fn adjacent(&self, m: usize, n: usize, min: u8, max: u8) -> Vec<Vertex> {
        let mut adj = Vec::new();
        let (i, j) = self.pos();
        if self.dir == 0 {
            if j > 0     { adj.push(Vertex::new2(*self, b'W')); }
            if j < n - 1 { adj.push(Vertex::new2(*self, b'E')); }
            if i > 0     { adj.push(Vertex::new2(*self, b'N')); }
            if i < m - 1 { adj.push(Vertex::new2(*self, b'S')); }
        } else {
            if self.count >= min {
                match self.dir {
                    b'N' | b'S' => {
                        if j > 0     { adj.push(Vertex::new2(*self, b'W')); }
                        if j < n - 1 { adj.push(Vertex::new2(*self, b'E')); }
                    },
                    b'E' | b'W' => {
                        if i > 0     { adj.push(Vertex::new2(*self, b'N')); }
                        if i < m - 1 { adj.push(Vertex::new2(*self, b'S')); }
                    },
                    _ => unreachable!(),
                }
            }
            if self.count < max {
                let v = Vertex::new2(*self, self.dir);
                match self.dir {
                    b'N' => { if i > 0     { adj.push(v); } },
                    b'S' => { if i < m - 1 { adj.push(v); } },
                    b'W' => { if j > 0     { adj.push(v); } },
                    b'E' => { if j < n - 1 { adj.push(v); } },
                    _ => unreachable!(),
                }
            }
        }
        adj
    }
}

/// Load and return the grid from the input file.
/// 
fn get_grid(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let grid   = lines.map(|line| Ok(line?.into_bytes()))
                      .collect::<Result<Vec<Vec<u8>>,Box<dyn Error>>>()?;
    Ok(grid)
}

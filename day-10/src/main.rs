//! Advent of Code 2023 Day 10 - Pipe Maze
//! 
//! Part 1: Find how many steps along the main loop it takes to get from start
//!         to the farthest point.
//! Part 2: Find how many tiles are enclosed in the path loop.

use std::collections::{HashSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::iter::once;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 10 Advent of Code\n"); 

    //let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    //let input = "./data/sample2.txt";
    //let input = "./data/sample3.txt";
    
    let start = Instant::now();

    part_1("./data/sample1.txt")?;
    part_2("./data/sample2.txt")?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Find how many steps along the main loop it takes to get from start
///         to the farthest point.
/// 
fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
    let mut maze  = Maze::new(path)?;
    let mut queue = maze.s_pipes()
                        .into_iter()
                        .map(|p| (p, 1))
                        .collect::<VecDeque<_>>();
    let mut max_d = 0;

    while let Some((pos, n_steps)) = queue.pop_front() {

        max_d = max_d.max(n_steps);
        
        maze.mark_visited(pos);
        
        if let Some(next) = maze.next_pipe(pos) {
            queue.push_back((next, n_steps + 1));
        }
    }
    println!("Part 1 Farthest Point.....: {}", max_d);

    Ok(())
}

/// Part 2: Find how many tiles are enclosed in the path loop.
/// 
/// UnionFind is utilized to group connected points. Those not within the path
/// loop will be a group that connects to the edges of the maze. Those within
/// the path loop will be a group that connects to itself.
/// 
/// The granularity of the maze is increased by a factor of 2 so there is 
/// guaranteed space between adjacent pipes. The path points are added to the 
/// UnionFind data structure with new midpoints added between them.
/// 
/// Then the points are iterated over, connecting adjacent points in the 
/// UnionFind structure if they aren't path points. When done, there should be 
/// three groups within the structure: one for the path points, one for the
/// points not enclosed in the path, and a group for those enclosed in it.
/// 
fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
    let mut maze  = Maze::new(path)?;
    let mut dims  = maze.get_dimensions();
    let mut start = maze.get_start();
    let mut uf    = UnionFind2D::new2d((dims.0 * 2, dims.1 * 2));
    let mut last  = maze.s_pipes()[0];
    let mut midp;

    maze.mark_visited(last);

    dims  = upscale(dims);
    start = upscale(start); 
    last  = upscale(last);
    midp  = midpoint(start, last);

    uf.union2d(start, last);
    uf.union2d(start, midp);
    
    // Keep scaling up the points on the matrix and generate artificial 
    // midpoints as the group of path points are added to UnionFind.
    while let Some(mut curr) = maze.next_pipe(dnscale(last)) {
        maze.mark_visited(curr);

        curr = upscale(curr);
        midp = midpoint(last, curr);

        uf.union2d(last, curr);
        uf.union2d(last, midp);

        last = curr;
    }
    uf.union2d(start, midpoint(start, last));

    let start_root = uf.find2d(start);

    // Now connect all adjacent points that aren't path points.
    connect_union_find_points(&mut uf, dims, start_root);

    // Get the UnionFind tile group counts.
    let tile_groups = get_union_find_group_counts(&uf, dims, start_root);
    
    // Produces 2 groups. One of them is the right one...
    println!("Part 2 Free Tile Groups...: {:?}", tile_groups);

    Ok(())
}

/// Connect all adjacent points that aren't path points.
/// 
fn connect_union_find_points(union_find : &mut UnionFind2D, 
                             dimensions : (usize, usize), 
                             start_root : (usize, usize)) 
{
    let dims = dimensions;
    let uf   = union_find;

    for i in 1..dims.0 - 1 {
        for j in 1..dims.1 - 1 {
            if uf.find2d((i, j)) != start_root {

                for p in [(i, j + 1), (i + 1, j), (i, j - 1), (i - 1, j)] {

                    if uf.find2d(p) != start_root {
                        uf.union2d((i, j), p);
                    }
                }
            }
        }
    }
}

/// Get the UnionFind tile group counts.
/// 
fn get_union_find_group_counts(union_find : &UnionFind2D, 
                               dimensions : (usize, usize), 
                               start_root : (usize, usize)) 
    -> Vec<usize> 
{
    let dims = dimensions;
    let uf   = union_find;
    let mut uf_groups = HashMap::new();

    for i in 1..dims.0 - 1 {
        for j in 1..dims.1 - 1 {
            // Skip odd points. These are the midpoints added during scaling.
            if i & 1 == 0 && j & 1 == 0 {

                let root = uf.find2d((i, j));

                if root != start_root {
                    *uf_groups.entry(root).or_insert(0) += 1;
                }
            }
        }
    }
    uf_groups.values().copied().collect::<Vec<_>>()
}

/// Upscale a point by a factor of 2.
/// 
fn upscale(p: (usize, usize)) -> (usize, usize) {
    (p.0 * 2, p.1 * 2)
}

/// Downscale a point by a factor of 2.
/// 
fn dnscale(p: (usize, usize)) -> (usize, usize) {
    (p.0 / 2, p.1 / 2)
}

/// Get the midpoint between two points.
/// 
fn midpoint(p: (usize, usize), q: (usize, usize)) -> (usize, usize) {
    ((p.0 + q.0) / 2, (p.1 + q.1) / 2)
}

/// Maze struct to hold the maze data and provide methods for traversing it.
/// 
struct Maze {
    maze  : Vec<Vec<char>>,
    start : (usize, usize),
    seen  : HashSet<(usize, usize)>,
}
impl Maze {
    fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let     file   = File::open(path)?;
        let     reader = BufReader::new(file);
        let     lines  = reader.lines();
        let mut maze   = Vec::new();
        let mut start  = (0, 0);

        for (i, line) in (1..).zip(lines) {
            let map_fn = |(j, c)| if c == 'S' { start = (i, j); c } else { c };
            let line   = line?;
            let row    = once('.').chain(line.chars())
                                  .chain(once('.'))
                                  .enumerate()
                                  .map(map_fn)
                                  .collect::<Vec<_>>();
            maze.push(row);
        }
        let pad = vec!['.'; maze[0].len()];
        
        maze.insert(0, pad.clone());
        maze.push(pad);

        Ok(Maze { maze, start, seen: HashSet::new() })
    }

    fn get_start(&self) -> (usize, usize) {
        self.start
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.maze.len(), self.maze[0].len())
    }

    fn mark_visited(&mut self, pos: (usize, usize)) {
        self.seen.insert(pos);
    }

    fn visited(&self, pos: (usize, usize)) -> bool {
        self.seen.contains(&pos)
    }

    fn next_pipe(&self, from : (usize, usize)) -> Option<(usize, usize)> {
        let pipe = self.maze[from.0][from.1];
        let opts = {
            match pipe {
                '|' => ((from.0 + 1, from.1), (from.0 - 1, from.1)),
                '-' => ((from.0, from.1 - 1), (from.0, from.1 + 1)),
                'L' => ((from.0 - 1, from.1), (from.0, from.1 + 1)),
                'J' => ((from.0 - 1, from.1), (from.0, from.1 - 1)),
                '7' => ((from.0 + 1, from.1), (from.0, from.1 - 1)),
                'F' => ((from.0 + 1, from.1), (from.0, from.1 + 1)),
                _   => panic!("Invalid pipe: {}", pipe),
            }
        };
        if self.can_connect(from, opts.0) && !self.visited(opts.0) {
            Some(opts.0)
        } else if self.can_connect(from, opts.1) && !self.visited(opts.1) {
            Some(opts.1)
        } else {
            None
        }
    }

    fn s_pipes(&self) -> Vec<(usize, usize)> {
        let mut next = Vec::new();
        let s = self.start;

        if self.can_connect(s, (s.0 + 1, s.1)) { next.push((s.0 + 1, s.1)); }
        if self.can_connect(s, (s.0 - 1, s.1)) { next.push((s.0 - 1, s.1)); }
        if self.can_connect(s, (s.0, s.1 + 1)) { next.push((s.0, s.1 + 1)); }
        if self.can_connect(s, (s.0, s.1 - 1)) { next.push((s.0, s.1 - 1)); }

        next
    }

    fn can_connect(&self, from : (usize, usize), to: (usize, usize)) -> bool {
        let fc = self.maze[from.0][from.1];
        let tc = self.maze[to.0][to.1];

        match fc {
            '|' => {
                (from.0 > to.0 && (tc == 'F' || tc == '7' || tc == '|')) || 
                (from.0 < to.0 && (tc == 'L' || tc == 'J' || tc == '|'))
            },
            '-' => {
                (from.1 < to.1 && (tc == '7' || tc == 'J' || tc == '-')) ||
                (from.1 > to.1 && (tc == 'L' || tc == 'F' || tc == '-'))
            },
            'L' => {
                (from.1 < to.1 && (tc == '-' || tc == 'J' || tc == '7')) ||
                (from.0 > to.0 && (tc == '|' || tc == '7' || tc == 'F'))
            },
            'J' => {
                (from.1 > to.1 && (tc == '-' || tc == 'L' || tc == 'F')) ||
                (from.0 > to.0 && (tc == '|' || tc == '7' || tc == 'F'))
            },
            '7' => {
                (from.1 > to.1 && (tc == '-' || tc == 'L' || tc == 'F')) ||
                (from.0 < to.0 && (tc == '|' || tc == 'L' || tc == 'J'))
            },
            'F' => {
                (from.1 < to.1 && (tc == '-' || tc == 'J' || tc == '7')) ||
                (from.0 < to.0 && (tc == '|' || tc == 'L' || tc == 'J'))
            },
            'S' => {
                (from.0 < to.0 && (tc == '|' || tc == 'J' || tc == 'L')) ||
                (from.0 > to.0 && (tc == '|' || tc == '7' || tc == 'F')) ||
                (from.1 < to.1 && (tc == '-' || tc == '7' || tc == 'J')) ||
                (from.1 > to.1 && (tc == '-' || tc == 'L' || tc == 'F'))
            },
            _   => { false },
        }
    }
}

/// UnionFind2D struct to hold the UnionFind data structure for 2D points.
/// It also supports uses cases for 1-D points.
/// 
struct UnionFind2D {
    link : Vec<usize>,
    size : Vec<usize>,
    dims : (usize, usize),
}
#[allow(dead_code)]
impl UnionFind2D {
    fn new(n: usize) -> Self {
        Self { link: (0..n).collect(), size: vec![1; n], dims: (n, 1) }
    }
    fn new2d(dims: (usize, usize)) -> Self {
        let n = dims.0 * dims.1;
        Self { link: (0..n).collect(), size: vec![1; n], dims }
    }
    fn find(&self, v : usize) -> usize {
        let mut v_root = v;
        while v_root != self.link[v_root] { 
            v_root = self.link[v_root]; 
        }
        v_root
    }
    fn size(&self, v : usize) -> usize {
        let v_root = self.find(v);
        self.size[v_root]
    }
    fn union(&mut self, u: usize, v: usize) {
        let u_root = self.find(u);
        let v_root = self.find(v);
        if u_root != v_root {
            if self.size[u_root] < self.size[v_root] {
                self.link[u_root] = v_root; 
                self.size[v_root] += self.size[u_root];
            } else {
                self.link[v_root] = u_root;
                self.size[u_root] += self.size[v_root];
            }
        }
    }
    fn size2d(&self, v : (usize, usize)) -> usize {
        let v_root = self.find(v.0 * self.dims.1 + v.1);
        self.size[v_root]
    }
    fn find2d(&self, v : (usize, usize)) -> (usize, usize) {
        let v_root = self.find(v.0 * self.dims.1 + v.1);
        (v_root / self.dims.1, v_root % self.dims.1)
    }
    fn union2d(&mut self, u: (usize, usize), v: (usize, usize)) {
        let u = u.0 * self.dims.1 + u.1;
        let v = v.0 * self.dims.1 + v.1;
        self.union(u, v);
    }
}

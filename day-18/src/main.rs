//! Advent of Code 2023 Day 18 - Lavaduct Lagoon
//! 
//! Part 1: The Elves are concerned the lagoon won't be large enough; if they 
//!         follow their dig plan, how many cubic meters of lava could it hold?
//! Part 2: Convert the hexadecimal color codes into the correct instructions; 
//!         if the Elves follow this new dig plan, how many cubic meters of lava
//!         could the lagoon hold?

use std::collections::{HashSet, BTreeSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

/// These states are used during calculations to determine how many tiles in a 
/// given column are filled.
/// 
#[derive(Clone, Copy)]
enum ColState<T> {
    Open(T),
    End(T),
    Closed,
} 
impl<T> ColState<T> {
    fn is_open(&self)   -> bool { matches!(self, ColState::Open(_)) }
    fn is_closed(&self) -> bool { matches!(self, ColState::Closed)  }
}

/// Part one or two of the puzzle.
/// 
#[derive(Debug)]
enum Part {
    One,
    Two,
}

type GridPoint = (i64, i64);
type GridLine  = (GridPoint, GridPoint);

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 18 Advent of Code\n"); 

    let start = Instant::now();

    for input in ["./data/sample1.txt", "./data/input1.txt"] {
        solution(input, Part::One)?;
        solution(input, Part::Two)?;
    }

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Solution for part one and two of the puzzle.
/// 
fn solution(path: &str, part: Part) -> Result<i64, Box<dyn Error>> {
    use ColState::*;
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let expr   = Regex::new(r"([UDLR]) +(\d+) +\(#(\w{5})(\w{1})\)")?;

    let mut horizontals = BTreeSet::new();
    let mut verticals   = HashSet::new();
    let mut cursor      = (0i64, 0i64); // (row, col), or (i, j)
    let mut up_left     = (0, 0);
    let mut lo_right    = (0, 0);

    // Parse the input data into sets of horizontal and vertical lines.
    for line in lines {
        let line = line?;
        let cap  = expr.captures(&line).ok_or("Invalid input")?;
        let (dir, dist) = {
            match part {
                Part::One => (cap[1].as_bytes()[0], cap[2].parse::<i64>()?),
                Part::Two => {
                    (cap[4].as_bytes()[0], i64::from_str_radix(&cap[3], 16)?)
                }}};
        match dir {
            b'U' | b'3' => {
                let p1 = (cursor.0 - dist, cursor.1);
                let p2 = (cursor.0, cursor.1);
                cursor.0 -= dist;
                up_left.0 = up_left.0.min(cursor.0);
                verticals.insert((p1, p2));
            },
            b'D' | b'1' => {
                let p1 = (cursor.0, cursor.1);
                let p2 = (cursor.0 + dist, cursor.1);
                cursor.0 += dist;
                lo_right.0 = lo_right.0.max(cursor.0);
                verticals.insert((p1, p2));
            },
            b'L' | b'2' => {
                let p1 = (cursor.0, cursor.1 - dist);
                let p2 = (cursor.0, cursor.1);
                cursor.1 -= dist;
                up_left.1 = up_left.1.min(cursor.1);
                horizontals.insert((p1, p2));
            },
            b'R' | b'0' => {
                let p1 = (cursor.0, cursor.1);
                let p2 = (cursor.0, cursor.1 + dist);
                cursor.1 += dist;
                lo_right.1 = lo_right.1.max(cursor.1);
                horizontals.insert((p1, p2));
            },
            _ => return Err("Invalid direction".into()),
        }
    }
    let _height = (lo_right.0 - up_left.0) as usize + 1;
    let width   = (lo_right.1 - up_left.1) as usize + 1;

    let mut capacity  = 0;
    let mut col_state = vec![Closed; width];

    macro_rules! col_state { 
        [$c:expr] => { (col_state[($c - up_left.1) as usize]) } 
    }

    let mut iter = horizontals.into_iter().peekable();

    // Iterate over the horizontal line end points.
    while let Some((mut p1, mut p2)) = iter.next() {
        let r1 = p1.0;
        loop { // on all lines in the same row.
            let (_, c1) = p1;
            let (_, c2) = p2;

            for c in c1..=c2 {
                match col_state![c] {
                    Open(r2) => {
                        if c == c1 || c == c2 {
                            col_state![c] = End(r1);
                        } else {
                            col_state![c] = Closed;
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
                            col_state![c] = Closed;
                            capacity += r1 - r2 - 1;
                        }
                    },
                    Closed => {
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
    println!("Part {:?} Lava Capacity: {}", part, capacity);
    Ok(capacity)
}

/// Prints the grid.
/// 
#[allow(dead_code, clippy::needless_range_loop)]
fn print_grid(horizontals : &BTreeSet<GridLine>,
              verticals   : &HashSet<GridLine>,
              height      : usize, 
              width       : usize,
              corner      : (i64, i64),
              reduce      : Option<usize>) 
{
    let reduce = reduce.unwrap_or(1);

    let mut grid = vec![vec!['.'; width / reduce + 1]; height / reduce + 1];

    for ((r1, c1), (r2, c2)) in horizontals.iter().chain(verticals) {
        let r1 = (r1 - corner.0) as usize / reduce;
        let c1 = (c1 - corner.1) as usize / reduce;
        let r2 = (r2 - corner.0) as usize / reduce;
        let c2 = (c2 - corner.1) as usize / reduce;

        if r1 == r2 {
            for c in c1..=c2 { grid[r1][c] = '#'; }
        } else {
            for r in r1..=r2 { grid[r][c1] = '#'; }
        }
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
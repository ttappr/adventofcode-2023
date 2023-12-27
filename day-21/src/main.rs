#![allow(unused)]
//! Advent of Code 2023 Day 21 - Step Counter
//! 
//! Part 1: Starting from the garden plot marked S on your map, how many garden 
//!         plots could the Elf reach in exactly 64 steps?
//! Part 2: 

use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 20 Advent of Code\n"); 

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

/// Part 1: Starting from the garden plot marked S on your map, how many garden 
///         plots could the Elf reach in exactly 64 steps?
/// 
fn part_1(path: &str) -> Result<i64, Box<dyn Error>> {
    let grid  = parse_input(path)?; 
    let start = grid.iter().enumerate()
                    .filter_map(|(i, row)| 
                                Some((i, row.iter().position(|&b| b == b'S')?)))
                    .next().ok_or("No 'S' found.")?;

    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen  = HashSet::new();
    let mut tiles = 0;

    const STEPS: usize = 64;

    while let Some((v1, step)) = queue.pop_front() {
        if !seen.contains(&v1) {
            seen.insert(v1);
            if (v1.0 + v1.1) & 1 == STEPS & 1 {
                tiles += 1;
            }
            if step <= STEPS {
                for v2 in [(v1.0.saturating_sub(1), v1.1), 
                           (v1.0, v1.1.saturating_sub(1)), 
                           (v1.0 + 1, v1.1), 
                           (v1.0, v1.1 + 1)] {
                    if grid.get(v2.0)
                           .and_then(|r| r.get(v2.1))
                           .map_or(false, |&b| b == b'.' || b == b'S') {
                            queue.push_back((v2, step + 1));
                    }
                }
            }
        }
    }
    println!("Part 1 Total plots: {}", tiles);
    Ok(tiles)
}

/// Part 2: 
/// 
fn part_2(path: &str) -> Result<i64, Box<dyn Error>> {
    let     grid  = parse_input(path)?; 
    let mut start = grid.iter().enumerate()
                    .filter_map(|(i, row)| 
                                Some((i, row.iter().position(|&b| b == b'S')?)))
                    .next().ok_or("No 'S' found.")?;

    let (m, n) = (grid.len(), grid[0].len());

    const STEPS: usize = 26501365;

    // Arbitrarily shift the start so coords will never be negative.
    start.0 += ((STEPS + m - 1) / m) * m; 
    start.1 += ((STEPS + n - 1) / n) * n;

    assert!(grid[start.0 % m][start.1 % n] == b'S');

    let mut queue  = VecDeque::from([(start, 0)]);
    let mut seen   = HashSet::new();
    let mut ltiles = Vec::new();
    let mut tiles  = 0;

    for i in 0..3 {
        // Number of steps to land in the unrocky diamond lanes on the map at
        // 3 successive scales. Points in the diamond are all equidistant from
        // S in terms of Manhattan distance. m / 2 steps reaches the first 
        // diamond. The number of steps seems to have been chosen to land us
        // in the diamond: (26501365 - 130 / 2) / 131 = 202300.00 Note that the 
        // result is the current year multilplied by 100.
        let steps = m / 2 + m * i; 

        while let Some((v1, step)) = queue.pop_front() {
            if !seen.contains(&v1) {
                seen.insert(v1);
                if (v1.0 + v1.1) & 1 == steps & 1 {
                    tiles += 1;
                }

                if step <= steps {
                    for v2 in [(v1.0 - 1, v1.1), 
                               (v1.0, v1.1 - 1), 
                               (v1.0 + 1, v1.1), 
                               (v1.0, v1.1 + 1)] {
                        
                        let b = grid[v2.0 % m][v2.1 % n];

                        if b == b'.' || b == b'S' {
                            queue.push_back((v2, step + 1));
                        }
                    }
                }
            }
        }
        ltiles.push(tiles);

        tiles = 0;
        seen.clear();
        queue.push_back((start, 0));
    }
    let a = (ltiles[2] - 2 * ltiles[1] + ltiles[0]) / 2;
    let b = ltiles[1] - ltiles[0] - a;
    let c = ltiles[0];
    let n = ((STEPS - m / 2) / m ) as i64;
    
    tiles = (a * n * n) + (b * n) + c;

    println!("Part 2 Total plots: {:?}", tiles);
    Ok(tiles)
}

/// Reads in the input file specified by path and parses it into a HashMap
/// holding the modules. The module identifiers are the keys.
///
fn parse_input(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();

    Ok(lines.map(|l| Ok(l?.bytes().collect()))
            .collect::<Result<Vec<Vec<u8>>, Box<dyn Error>>>()?)
}

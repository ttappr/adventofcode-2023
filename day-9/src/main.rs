//! Advent of Code 2023 Day 9 - Mirage Maintenance
//! 
//! Part 1: Analyze your OASIS report and extrapolate the next value for each 
//!         history.
//! Part 2: Analyze your OASIS report again, this time extrapolating the 
//!         previous value for each history.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 8 Advent of Code\n"); 

    let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

macro_rules! vlast  { ($v:expr) => { ($v.last().unwrap_or(&vec![])) } }
macro_rules! nlast  { ($v:expr) => { ($v.last().unwrap_or(&0))      } }
macro_rules! nfirst { ($n:expr) => { ($n.first().unwrap_or(&0))     } }

/// Part 1: Analyze your OASIS report and extrapolate the next value for each 
///         history.
/// 
fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
    let     file   = File::open(path)?;
    let     reader = BufReader::new(file);
    let     lines  = reader.lines();
    let mut total  = 0;

    for line in lines {
        let     line = line?;
        let mut rows = vec![line.split_whitespace()
                                .map(str::parse)
                                .collect::<Result<Vec<i32>,_>>()?];

        while vlast!(rows).iter().any(|&x| x != 0) {
            let row = vlast!(rows).windows(2)
                                  .map(|p| p[1] - p[0])
                                  .collect::<Vec<_>>();
            rows.push(row);
        }
        total += rows.iter().map(|r| nlast!(r)).sum::<i32>();
    }
    println!("Part 1 Last Column Total: {}", total);
    Ok(())
}

/// Part 2: Analyze your OASIS report again, this time extrapolating the 
///         previous value for each history.
/// 
fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
    let     file   = File::open(path)?;
    let     reader = BufReader::new(file);
    let     lines  = reader.lines();
    let mut total  = 0;

    for line in lines {
        let     line = line?;
        let mut val  = 0;
        let mut rows = vec![line.split_whitespace()
                                .map(str::parse)
                                .collect::<Result<Vec<i32>,_>>()?];

        while vlast!(rows).iter().any(|&x| x != 0) {
            let row = vlast!(rows).windows(2)
                                  .map(|p| p[1] - p[0])
                                  .collect::<Vec<_>>();
            rows.push(row);
        }
        rows.iter().rev().for_each(|r| val = nfirst!(r) - val);
        total += val;
    }
    println!("Part 2 First Column Total: {}", total);
    Ok(())
}

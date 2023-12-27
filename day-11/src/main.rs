//! Advent of Code 2023 Day 11 - Cosmic Expansion
//! 
//! Part 1 & 2: Get the sum of the taxi distances between galaxies given an
//!             expansion factor applied to empty columns and rows.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::time::Instant;
use fenwick::Fenwick;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 11 Advent of Code\n"); 

    let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    
    let start = Instant::now();

    solution2(input, 2)?;
    solution2(input, 1_000_000)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1 & 2: Get the sum of the taxi distances between galaxies given an
///             expansion factor applied to empty columns and rows.
///             This version is 5x faster than solution2.
/// 
#[allow(dead_code)]
fn solution1(path: &str, expansion: i64) -> Result<(), Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let lines  = reader.lines();

    let mut col_pop  = Vec::new();
    let mut row_pop  = Vec::new();
    let mut galaxies = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line?;
        row_pop.push(0);
        col_pop.resize(line.len(), 0);

        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                row_pop[i] += 1;
                col_pop[j] += 1;
                galaxies.push((i, j));
            }
        }
    }
    // Prefix sum arrays.
    let mut row_dist = Vec::new();
    let mut col_dist = Vec::new();
    let mut sum      = 0;

    for pop in row_pop {
        if pop == 0 { sum += expansion; }
        else        { sum += 1; }
        row_dist.push(sum);
    }
    sum = 0;
    for pop in col_pop {
        if pop == 0 { sum += expansion; }
        else        { sum += 1; }
        col_dist.push(sum);
    }
    sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            
            // Taxi distance.
            sum += (row_dist[g1.0] - row_dist[g2.0]).abs() +
                   (col_dist[g1.1] - col_dist[g2.1]).abs();
        }
    }
    println!("Sum of shortest paths: {:>12}", sum);
    Ok(())
}

/// Part 1 & 2: Get the sum of the taxi distances between galaxies given an
///             expansion factor applied to empty columns and rows.
/// 
#[allow(dead_code)]
fn solution2(path: &str, expansion: i64) -> Result<(), Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let lines  = reader.lines();

    let mut col_dists = Fenwick::from(vec![expansion; 256]);
    let mut row_dists = Fenwick::from(vec![expansion; 256]);
    let mut galaxies  = Vec::new();

    for (i, line) in lines.enumerate() {
        let line = line?;
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push((i, j));
                row_dists.set(i, 1);
                col_dists.set(j, 1);
            }
        }
    }
    let mut sum = 0;

    for (i, g1) in (1..).zip(&galaxies) {
        for g2 in &galaxies[i..] {
            // Taxi distance.
            sum += row_dists.range_sum2(g1.0.min(g2.0), g2.0.max(g1.0)) +
                   col_dists.range_sum2(g1.1.min(g2.1), g2.1.max(g1.1));
        }
    }
    println!("Sum of shortest paths: {:>12}", sum);
    Ok(())
}


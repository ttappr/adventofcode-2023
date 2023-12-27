//! Advent of Code 2023 Day 13 - Point of Incidence
//! 
//! Part 1: Find the line of reflection in each of the patterns in your notes.
//! Part 2: In each pattern, fix the smudge and find the different line of 
//!         reflection.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 13 Advent of Code\n"); 

    //let input = "./data/input1.txt";
    let input = "./data/sample1.txt";
    //let input = "./data/sample2.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Find the line of reflection in each of the patterns in your notes.
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut counter  = HashMap::new();
    let     matrices = get_matrices(path)?;
    let mut total    = 0;

    for mut m in matrices {
        let mut done = false;
        counter.clear();

        for r in &m {
            for p in palindromes(r) {
                *counter.entry(p).or_insert(0) += 1;
            }
        }
        for (p, &c) in &counter {
            if c == m.len() {
                total += p.1;
                done = true;
                break;
            }
        }
        if !done {
            m = rot90_counter_clockwise(&m);
            counter.clear();
            for r in &m {
                for p in palindromes(r) {
                    *counter.entry(p).or_insert(0) += 1;
                }
            }
            for (p, &c) in &counter {
                if c == m.len() {
                    total += p.1 * 100;
                    break;
                }
            }
        }
    }

    println!("Part 1 Total: {}", total);
    Ok(total)
}

/// Part 2: In each pattern, fix the smudge and find the different line of 
///         reflection.
/// 
fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut counter  = HashMap::new();
    let     matrices = get_matrices(path)?;
    let mut total    = 0;
    
    for mut m in matrices {
        let mut done = false;
        counter.clear();

        for r in &m {
            for p in palindromes(r) {
                *counter.entry(p).or_insert(0) += 1;
            }
        }
        for (p, &c) in &counter {
            if c == m.len() - 1 {
                total += p.1;
                done = true;
                break;
            }
        }
        if !done {
            m = rot90_counter_clockwise(&m);
            counter.clear();

            for r in &m {
                for p in palindromes(r) {
                    *counter.entry(p).or_insert(0) += 1;
                }
            }
            for (p, &c) in &counter {
                if c == m.len() - 1 {
                    total += p.1 * 100;
                    break;
                }
            }
        }
    }
    println!("Part 2 Total: {}", total);
    Ok(total)
}

/// Reads the input file and returns a vector of matrices.
/// 
fn get_matrices(path: &str) -> Result<Vec<Vec<Vec<u8>>>, Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let lines  = reader.lines();
    
    let mut matrices = Vec::new();
    let mut matrix   = Vec::new();
    
    for line in lines {
        let line = line?;
        if line.is_empty() {
            matrices.push(matrix);
            matrix = Vec::new();
        } else {
            matrix.push(line.into_bytes());
        }
    }
    matrices.push(matrix);

    Ok(matrices)
}

/// Rotates matrix 90 degrees counterclockwise.
/// 
fn rot90_counter_clockwise(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut matrix2 = vec![vec![0; m]; n];

    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        for j in 0..n {
            matrix2[n - j - 1][i] = matrix[i][j];
        }
    }
    matrix2
}

/// This is Manacher's algorithm, but instead of just returning the max, it 
/// returns all non-zero palindromes. The return data is a vector of tuples, 
/// where the first element is the length of the palindrome, and the second 
/// element is the index of the center of the palindrome.
/// 
fn palindromes(s: &[u8]) -> Vec<(usize, usize)> {
    use Ordering::*;
    let (_m, n)    = (s.len(), s.len() * 2 + 1);
    let mut radii = vec![0; n];
    let mut ctr   = 0;
    let mut rad   = 0;

    macro_rules! s2 {
        [$i: expr] => { ( if $i & 1 == 0 { b'|' } else { s[$i / 2] } ) }
    }

    while ctr < n {
        while ctr > rad + 1
            && ctr + rad + 1 < n 
            && s2![ctr - rad - 1] == s2![ctr + rad + 1] {
                rad += 1;
        }
        radii[ctr] = rad;

        let last_ctr = ctr;
        let last_rad = rad;

        ctr += 1;
        rad  = 0;

        while ctr <= last_ctr + last_rad {
            let mirr_ctr     = last_ctr + last_ctr - ctr;
            let max_mirr_rad = last_ctr + last_rad - ctr;

            match radii[mirr_ctr].cmp(&max_mirr_rad) {
                Less    => { radii[ctr] = radii[mirr_ctr]; ctr += 1; },
                Greater => { radii[ctr] = max_mirr_rad   ; ctr += 1; },
                Equal   => { rad = max_mirr_rad; break; },
            }
        }
    }
    radii.into_iter().step_by(2).enumerate()
         .map(|(i, r)| ((r + 1) / 2, i))
         .filter(|&(r, _)| r > 0)
         .collect()
}

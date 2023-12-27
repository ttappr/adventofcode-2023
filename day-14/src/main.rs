//! Advent of Code 2023 Day 14 - Parabolic Reflector Dish
//! 
//! Part 1: Tilt the platform so that the rounded rocks all roll north and 
//!         calculate the total load on the north support beams.
//! Part 2: Run the spin cycle for 1000000000 cycles. Afterward, get the total
//!         load on the north support beams.


use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 14 Advent of Code\n"); 

    //let input = "./data/input1.txt";
    let input = "./data/sample1.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Tilt the platform so that the rounded rocks all roll north and 
///         calculate the total load on the north support beams.
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let mut grid = get_grid(path)?;
    
    roll_n(&mut grid);

    let total = get_total_load(&grid);

    println!("Part 1 Total: {}", total);
    Ok(total)
}

/// Part 2: Run the spin cycle for 1000000000 cycles. Afterward, get the total
///         load on the north support beams.
/// 
fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
    const N_CYCLES    : usize = 1_000_000_000;
    const N_TEST_VALS : usize = 256;

    let mut grid   = get_grid(path)?;
    let mut hashes = Vec::new(); 

    // Generate a vector of hashes to use for cycle detection.
    for _ in 0..N_TEST_VALS {
        roll_n(&mut grid);
        roll_w(&mut grid);
        roll_s(&mut grid);
        roll_e(&mut grid);
        hashes.push(grid_hash(&grid));
    }
    // Find the cycle length and start index of the cycle.
    let (lam, mu) = brent((hashes[0], 1), |(_, i)| (hashes[*i], (*i + 1)));

    // Calculate the cycles needed, get a fresh grid and run the cycles.
    let n_cyc = (N_CYCLES - mu - 1) % lam + mu + 1;

    grid = get_grid(path)?;

    for _ in 0..n_cyc {
        roll_n(&mut grid);
        roll_w(&mut grid);
        roll_s(&mut grid);
        roll_e(&mut grid);
    }
    
    let total = get_total_load(&grid);

    println!("Part 2 Total: {}", total);
    Ok(total)
}

/// Get the total load on the north support beams.
/// 
fn get_total_load(grid: &Vec<Vec<u8>>) -> usize {
    let mut total = 0;
    
    for (r, row) in (1..=grid.len()).rev().zip(grid) {
        for &ch in row {
            if ch == b'O' {
                total += r;
            }
        }
    }
    total
}

/// Load and return the grid from the input file.
/// 
fn get_grid(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let grid   = lines.map(|line| line.unwrap()
                      .into_bytes())
                      .collect::<Vec<_>>();
    Ok(grid)
}

/// Roll the grid north.
/// 
fn roll_n(grid: &mut Vec<Vec<u8>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut heights = vec![0; n];
    let mut k;

    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                b'#' => {
                    heights[j] = i + 1;
                },
                b'O' => {
                    k = heights[j];
                    heights[j] += 1;
                    grid[i][j] = b'.';
                    grid[k][j] = b'O';
                },
                _   => (),
            }
        }
    }
}

/// Roll the grid west.
/// 
fn roll_w(grid: &mut Vec<Vec<u8>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut heights = vec![0; m];
    let mut k;

    for j in 0..n {
        for i in 0..m {
            match grid[i][j] {
                b'#' => {
                    heights[i] = j + 1;
                },
                b'O' => {
                    k = heights[i];
                    heights[i] += 1;
                    grid[i][j] = b'.';
                    grid[i][k] = b'O';
                },
                _   => (),
            }
        }
    }
}

/// Roll the grid south.
/// 
fn roll_s(grid: &mut Vec<Vec<u8>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut heights = vec![m - 1; n];
    let mut k;

    #[allow(clippy::needless_range_loop)]
    for i in (0..m).rev() {
        for j in 0..n {
            match grid[i][j] {
                b'#' => {
                    heights[j] = i.saturating_sub(1);
                },
                b'O' => {
                    k = heights[j];
                    heights[j] = heights[j].saturating_sub(1);
                    grid[i][j] = b'.';
                    grid[k][j] = b'O';
                },
                _   => (),
            }
        }
    }
}

/// Roll the grid east.
/// 
fn roll_e(grid: &mut Vec<Vec<u8>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut heights = vec![n - 1; m];
    let mut k;

    for j in (0..n).rev() {
        for i in 0..m {
            match grid[i][j] {
                b'#' => {
                    heights[i] = j.saturating_sub(1);
                },
                b'O' => {
                    k = heights[i];
                    heights[i] = heights[i].saturating_sub(1);
                    grid[i][j] = b'.';
                    grid[i][k] = b'O';
                },
                _   => (),
            }
        }
    }
}

/// Hash the grid.
/// 
fn grid_hash(grid: &Vec<Vec<u8>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}

/// Pretty prints the grid.
/// 
#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
    println!();
}

/// Brent's Algorithm - finds the length of a cycle in a sequence and the start 
/// index of the cycle. The return value is of the form (lam, mu) where lam is
/// the length of the cycle and mu is the start index of the cycle.
/// 
fn brent<F, T, I>(x0: (T, I), f: F) -> (usize, usize) 
where
    F: Fn(&(T, I)) -> (T, I),
    T: PartialEq + Clone,
    I: Clone,
{
    let mut power = 1;
    let mut lam   = 1;
    let mut tort  = x0.clone();
    let mut hare  = f(&x0);
    let mut mu    = 0;

    while tort.0 != hare.0 {
        if power == lam {
            power *= 2;
            tort   = hare.clone();
            lam    = 0;
        }
        hare = f(&hare);
        lam += 1;
    }
    tort = x0.clone();
    hare = x0.clone();

    for _ in 0..lam {
        hare = f(&hare);
    }
    while tort.0 != hare.0 {
        tort = f(&tort);
        hare = f(&hare);
        mu += 1;
    }
    (lam, mu)
}

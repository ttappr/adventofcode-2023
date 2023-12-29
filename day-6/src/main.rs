//! Advent of Code 2023 Day 6 - Wait For It
//! 
//! part 1: Calculate the product of the number of t_hold times that win each of
//!         the races.
//! Part 2: Given one race with large parameters, calculate the number of t_hold
//!         times that win.

use std::fs::read_to_string;
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 6 Advent of Code\n");

    //let input = "./data/input1.txt";
    let input = "./data/sample1.txt";
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// part 1: Calculate the product of the number of t_hold times that win each of
///         the races.
/// 
fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
    let lines = get_lines(path)?;
    let times = get_f64_vec(&lines[0][10..])?;
    let dists = get_f64_vec(&lines[1][10..])?;

    let mut prod = 1.0;

    for (&t_r, &d) in times.iter().zip(&dists) {

        let t_h1 = (t_r - (t_r.powf(2.0) - 4.0 * d).sqrt()) / 2.0;
        let t_h2 = (t_r + (t_r.powf(2.0) - 4.0 * d).sqrt()) / 2.0;

        prod *= t_h2.ceil() - t_h1.floor() - 1.0;
    }
    println!("Part 1, product: {}", prod);

    Ok(())
}

/// Part 2: Given one race with large parameters, calculate the number of t_hold
///         times that win.
/// 
fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
    let lines = get_lines(path)?;
    let times = &lines[0][10..];
    let dists = &lines[1][10..];

    let t_r  = get_joined_f64(times)?;
    let d    = get_joined_f64(dists)?;

    let t_h1 = (t_r - (t_r.powf(2.0) - 4.0 * d).sqrt()) / 2.0;
    let t_h2 = (t_r + (t_r.powf(2.0) - 4.0 * d).sqrt()) / 2.0;

    println!("Part 2, ways to win: {}", t_h2.floor() - t_h1.ceil() + 1.0);

    Ok(())
} 

fn get_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(read_to_string(path)?.lines()
                            .map(|s| s.into())
                            .collect::<Vec<String>>())
}

fn get_f64_vec(s: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    Ok(s.split_whitespace().map(|x| x.parse())
                           .collect::<Result<Vec<f64>,_>>()?)
}

fn get_joined_f64(s: &str) -> Result<f64, Box<dyn Error>> {
    Ok(s.split_whitespace().collect::<String>().parse::<f64>()?)
}
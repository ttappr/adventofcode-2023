//! Advent of Code, Day 2
//! Part 1: Cube Conundrum. Determine which games would have been possible.
//! Part 2: Find minimum set of cubes needed for each game.
//! 

use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 2 Advent of Code");
    part_1()?;
    part_2()?;
    Ok(())
}

/// Part 1: Cube Conundrum. Determine which games would have been possible.
/// 
fn part_1() -> Result<(), Box<dyn Error>> {
    let file   = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let expr1  = Regex::new(r"^Game (\d+)")?;
    let expr2  = Regex::new(r"(\d+) (red|green|blue)")?;

    let mut map   = HashMap::new();
    let mut total = 0;

    for line in reader.lines() {
        let line  = line?;
        let id    = expr1.captures(line.as_str()).unwrap()[1].parse::<i32>()?;
        let found = expr2.captures_iter(line.as_str());

        for finded in found {
            let num   = finded[1].parse::<i32>()?;
            let color = finded[2].to_string();
            let count = map.entry(color).or_insert(num);

            *count = num.max(*count);
        }
        if map.get("red")   <= Some(&12) &&
           map.get("green") <= Some(&13) &&
           map.get("blue")  <= Some(&14) {

            total += id;
        }
        map.clear();
    }
    println!("Part 1 Total: {}", total);
    Ok(())
}

/// Part 2: Find minimum set of cubes needed for each game.
/// 
fn part_2() -> Result<(), Box<dyn Error>> {
    let file   = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let expr2  = Regex::new(r"(\d+) (red|green|blue)")?;

    let mut map   = HashMap::new();
    let mut total = 0;

    for line in reader.lines() {
        let line  = line?;
        let found = expr2.captures_iter(line.as_str());

        for finded in found {
            let num   = finded[1].parse::<i32>()?;
            let color = finded[2].to_string();
            let count = map.entry(color).or_insert(num);

            *count = num.max(*count);
        }
        total += map.get("red").unwrap_or(&0) *
                 map.get("green").unwrap_or(&0) *
                 map.get("blue").unwrap_or(&0);
        map.clear();
    }
    println!("Part 2 Total: {}", total);
    Ok(())
}
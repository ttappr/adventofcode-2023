//! Advent of Code 2023 Day 8 - Haunted Wasteland
//! 
//! Part 1: Navigate from AAA to ZZZ following the steps.
//! Part 2: Waste a whole day chasing a rabbit down a hole, then realise the
//!         solution really was the last idea you thought was too dumb to try.

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use num::integer::lcm;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 8 Advent of Code\n"); 

    let start = Instant::now();

    //part_1("./data/input1.txt")?;
    //part_2("./data/input1.txt")?;

    part_1("./data/sample1.txt")?;
    part_2("./data/sample3.txt")?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Navigate from AAA to ZZZ following the steps.
/// 
fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
    let     file    = File::open(path)?;
    let     reader  = BufReader::new(file);
    let mut lines   = reader.lines();
    let     expr    = Regex::new(r"(\w+) += +\((\w+), +(\w+)\)")?;
    let mut map     = HashMap::new();
    let     dirs    = lines.next().ok_or("No lines in file")??;
    let mut curr    = "AAA".to_string();
    let mut n_steps = 0;

    lines.next().ok_or("Bad file format.")??;

    for line in lines {
        let line  = line?;
        let caps  = expr.captures(&line).ok_or("Bad file format.")?;
        let key   = caps[1].to_string();
        let left  = caps[2].to_string();
        let right = caps[3].to_string();

        map.insert(key, (left, right));
    }

    for dir in dirs.chars().cycle() {
        let (left, right) = map.get(&curr).ok_or("Bad map!")?;

        if dir == 'L' { curr = left.to_string();  } 
        else          { curr = right.to_string(); }
        n_steps += 1;

        if curr == "ZZZ" { break; }
    }

    println!("Part 1 Total Steps: {}", n_steps);

    Ok(())
}

/// Part 2: Start from several points on the map, find the lengths of cycles
///         from each. The answer is the least common multiple of all the
///         cycle lengths. Bleh!
/// 
fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
    let     file    = File::open(path)?;
    let     reader  = BufReader::new(file);
    let mut lines   = reader.lines();
    let     expr    = Regex::new(r"(\w+) += +\((\w+), +(\w+)\)")?;
    let mut map     = HashMap::new();
    let     dirs    = lines.next().ok_or("No lines in file")??;
    let mut starts  = Vec::new();
    let mut cyc_lcm = 1;

    lines.next().ok_or("Bad file format.")??;

    for line in lines {
        let line  = line?;
        let caps  = expr.captures(&line).ok_or("Bad file format.")?;
        let key   = caps[1].to_string();
        let left  = caps[2].to_string();
        let right = caps[3].to_string();

        map.insert(key.clone(), (left, right));

        if key.as_bytes()[2] == b'A' { starts.push(key); }
    }
    for start in starts {
        let access_fn = |k: &((_,usize), _)| {
            let (l, r) = map.get(&k.0.0).unwrap();

            if dirs.as_bytes()[k.1] == b'L' 
                 { ((l.clone(), (k.0.1 + 1) % dirs.len()), 0) } 
            else { ((r.clone(), (k.0.1 + 1) % dirs.len()), 0) }
        };
        let (lam, _) = brent(((start.clone(), 0), 0), access_fn);

        cyc_lcm = lcm(cyc_lcm, lam);
    }

    println!("Part 2 Total Steps: {}", cyc_lcm);

    Ok(())
}

/// Brent's Algorithm - finds the length of a cycle in a sequence and the start 
/// index of the cycle. The return value is of the form (lam, mu) where lam is
/// the length of the cycle and mu is the start index of the cycle.
/// 
/// In the tuple pattern `(T, I)`, `T` represents items of the sequence, while 
/// `I` (Ignored) is not used internally by the algorithm, so `I` can be used as 
/// anything by the caller. Only `T` is used for comparisons.
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

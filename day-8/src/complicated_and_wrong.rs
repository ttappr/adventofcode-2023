#![allow(unused)]
//! Advent of Code 2023 Day 8 - Haunted Wasteland
//! 
//! Part 1: Navigate from AAA to ZZZ following the steps.
//! Part 2: 

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::time::Instant;
use num::integer::lcm;
use regex::Regex;

#[derive(Debug, Clone)]
struct Cycle {
    start : String, // Start key.
    lam   : usize,  // Length of cycle.
    pos_z : usize,  // Position of Z end point in cycle.
    cyc_i : usize,  // Current position in cycle.
    mu    : usize,  // Start index of cycle.
}
impl Cycle {
    fn new(start : String, 
           dirs  : &str, 
           map   : &HashMap<String, (String, String)>) 
        -> Self
    {
        let access_fn = |k: &(_, _)| {
            let (l, r) = map.get(&k.0).unwrap();

            if dirs.as_bytes()[k.1] == b'L' 
                 { (l.clone(), (k.1 + 1) % dirs.len()) } 
            else { (r.clone(), (k.1 + 1) % dirs.len()) }
        };

        let (lam, mu) = brent((start.clone(), 0), access_fn);

        let mut key   = start.clone();
        let mut pos_z = 0;

        for i in 0..lam + mu {

            if key.as_bytes()[2] == b'Z' { 
                pos_z = i - mu; 
                break;
            }
            let (l, r) = map.get(&key).unwrap().clone();

            if dirs.as_bytes()[i % dirs.len()] == b'R' 
                 { key = r; } 
            else { key = l; }
        }
        Cycle { start, lam, mu, pos_z, cyc_i: 0 }
    }/*
    fn validate(&self,
                dirs : &str, 
                map  : &HashMap<String, (String, String)>) 
        -> bool
    {
        let mut hash = 
        let mut key = self.start.clone();

        for i in 0..self.mu {
            let (l, r) = map.get(&key).unwrap().clone();

            if dirs.as_bytes()[i] == b'R' 
                 { key = r; } 
            else { key = l; }
        }
        for i in self.mu..self.lam * 3 {
            let (l, r) = map.get(&key).unwrap().clone();

            if dirs.as_bytes()[i % dirs.len()] == b'R' 
                 { key = r; } 
            else { key = l; }
        }
        false
    }*/
    
    fn incr(&mut self) {
        if self.mu > 0 {
            self.mu -= 1;
        } else {
            self.cyc_i += 1;
            self.cyc_i %= self.lam;
        }
    }
    fn incr_by(&mut self, n: usize) {
        if self.mu > 0 {
            if n > self.mu {
                self.cyc_i += n - self.mu;
                self.cyc_i %= self.lam;
                self.mu = 0;
            } else {
                self.mu -= n; 
            }
        } else {
            self.cyc_i += n;
            self.cyc_i %= self.lam;
        }
    }
    fn at_z(&self) -> bool {
        self.cyc_i == self.pos_z
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 8 Advent of Code\n"); 

    let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    //let input = "./data/sample2.txt";
    //let input = "./data/sample3.txt";
    
    let start = Instant::now();

    //part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Navigate from AAA to ZZZ following the steps.
/// 
#[allow(dead_code)]
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

fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
    let     file    = File::open(path)?;
    let     reader  = BufReader::new(file);
    let mut lines   = reader.lines();
    let     expr    = Regex::new(r"(\w+) += +\((\w+), +(\w+)\)")?;
    let mut map     = HashMap::new();
    let     dirs    = lines.next().ok_or("No lines in file")??;
    let mut starts  = Vec::new();
    let mut cycles  = Vec::new();

    let mut n_steps = 0_u128;

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
        cycles.push(Cycle::new(start, &dirs, &map));
    }

    println!("{:#?}", cycles);

    //cycles.remove(0);

    for cycle in &mut cycles {
        cycle.incr_by(6235825343);
        //cycle.incr_by(355428070777838604)
    }
    n_steps += 6235825343;
    //n_steps += 355428070777838604;

    while !cycles.iter().all(|c| c.at_z()) {
        for cycle in &mut cycles {
            cycle.incr_by(1);
            //cycle.incr_by(27849062);
        }
        //n_steps += 27849062;
        n_steps += 1;
    }

    println!("Part 2 Total Steps: {}", n_steps);

    Ok(())
}

/// Brent's Algorithm.
/// 
/// Finds the length of a cycle in a sequence and the start index of the cycle.
/// 
/// `x0` is a tuple of a sequence value and callback data. The callback data 
/// is ignored and remains unchanged. The caller determines how it's used.
/// 
/// The function `f` maps the current value to the next. It takes a tuple with
/// the same format and fields as `x0`. Only the first value is used internally
/// for comparison.
/// 
/// The first value, `lam`, of the returned tuple is the length of the cycle, 
/// found and the second value, `mu` is the start index of the cycle in the
/// sequence.
/// 
fn brent<F, T, I>(x0: (T, I), f: F) -> (usize, usize) 
where
    F: Fn(&(T, I)) -> (T, I),
    T: PartialEq + Clone,
    I: PartialEq + Clone,
{
    let mut power = 1;
    let mut lam   = 1;
    let mut tort  = x0.clone();
    let mut hare  = f(&x0);
    let mut mu    = 0;

    while tort != hare {
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
    while tort != hare {
        tort = f(&tort);
        hare = f(&hare);
        mu += 1;
    }
    (lam, mu)
}

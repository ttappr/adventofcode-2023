//! Advent of Code 2023 Day 15 - Lens Library
//! 
//! Part 1: Run the HASH algorithm on each step in the initialization sequence 
//!         and report the sum of the results.
//! Part 2: Populate the lense boxes, then calculate the focusing power of the
//!         resulting lens configuration

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 15 Advent of Code\n"); 

    let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Run the HASH algorithm on each step in the initialization sequence 
///         and report the sum of the results.
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let reader  = ElfReader::new(path, ',')?;
    let mut sum = 0;

    for record in reader {
        sum += hash(record?.as_bytes());
    }

    println!("Part 1 Sum of Results ........ {}", sum);
    Ok(sum)
}

/// Part 2: Populate the lense boxes, then calculate the focusing power of the
///         resulting lens configuration
/// 
fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
    let     s2int  = |s: &str| s.parse::<usize>();
    let     reader = ElfReader::new(path, ',')?;
    let     expr   = Regex::new(r"(\w+)([=-])(\d+)?")?;
    let mut boxes  = vec![vec![]; 256];
    let mut total  = 0;
    
    for record in reader {
        let rec   = record?;
        let caps  = expr.captures(&rec).ok_or("Bad record!")?;

        let label = caps[1].to_string();
        let oper  = &caps[2];
        let box_  = hash(label.as_bytes());
        let pos   = boxes[box_].iter().position(|lf: &(_,_)| lf.0 == label);

        match (oper, pos) {
            ("=", Some(p)) => { boxes[box_][p].1 = s2int(&caps[3])?; },
            ("=", None)    => { boxes[box_].push((label, s2int(&caps[3])?)); },
            ("-", Some(p)) => { boxes[box_].remove(p); },
            _              => (),
        }
    }
    for (box_n, box_) in (1..).zip(&boxes) {
        for (slot_n, &(_, focal_len)) in (1..).zip(box_) {

            total += focal_power(box_n, slot_n, focal_len);
        }
    }
    println!("Part 2 Total Focusing Power .. {}", total);

    Ok(total)
}

/// Calculate the focusing power of a lens.
/// 
fn focal_power(box_n: usize, slot: usize, focal_len: usize) -> usize {
    box_n * slot * focal_len
}

/// Calculate the hash of a lense label used to lcoate which box it goes in.
/// 
fn hash(s: &[u8]) -> usize {
    let mut h = 0;
    for &b in s {
        h += b as usize;
        h *= 17;
        h %= 256;
    }
    h
}

/// A custom buffered reader that reads in strings up to the delimiter 
/// character. Memory efficiency is the objective of this reader.
/// 
struct ElfReader {
    reader    : BufReader<File>,
    delimiter : u8,
    buffer    : Vec<u8>,
}
impl ElfReader {
    fn new(path: &str, delimiter: char) -> Result<Self, Box<dyn Error>> {
        let reader = BufReader::new(File::open(path)?);
        let buffer = Vec::new();
        Ok(Self { reader, buffer, delimiter: delimiter as u8 })
    }
}
impl Iterator for ElfReader {
    type Item = Result<String, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_until(self.delimiter, &mut self.buffer) {
            Ok(0) => None,
            Ok(_) => {
                if self.buffer.last() == Some(&self.delimiter) {
                    self.buffer.pop();
                }
                if let Ok(s) = String::from_utf8(self.buffer.clone()) {
                    self.buffer.clear();
                    Some(Ok(s))
                } else {
                    Some(Err("Bad record!".into()))
                }
            },
            Err(e) => Some(Err(e.into())),
        }
    }
}

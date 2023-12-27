//! Advent of Code, Day 1: Trebuchetf?!
//! Part 1, Get the sum of all calibration values from input data.
//! Part 2, Get the som of all calibration values considering numbers spelled 
//!         out.

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    part_1()?;
    part_2()?;
    Ok(())
}

/// Part 1, Get the sum of all calibration values from input data.
/// 
fn part_1() -> Result<(), Box<dyn Error>> {
    let file   = File::open("./data/sample1.txt")?;    
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let     line  = line?;
        let mut chars = line.chars()
                            .filter(|c| c.is_ascii_digit())
                            .collect::<Vec<_>>();
        if chars.len() == 1 {
            chars.push(chars[0]);
        }
        let num = [chars[0], *chars.last().unwrap()]
                  .into_iter().collect::<String>();

        total += num.parse::<i32>()?;
    }
    println!("Part 1 Total: {}", total);

    Ok(())
}

/// Part 2, Get the som of all calibration values considering numbers spelled
/// out.
/// 
fn part_2() -> Result<(), Box<dyn Error>> {
    let file   = File::open("./data/sample2.txt")?;    
    let reader = BufReader::new(file);
    let rexpr  = Regex::new("one|two|three|four|five|six|seven|\
                                eight|nine|[0-9]")?;
    let mut total = 0;

    for line in reader.lines() {
        let     line   = line?;
        let mut number = 0;
        let     caps   = rexpr.find_iter(line.as_str())
                              .map(|m| m.as_str())
                              .collect::<Vec<_>>();
        let digit1 = *caps.first().unwrap();
        let digit2 = *caps.last().unwrap_or(&digit1);

        for num in [digit1, digit2] {
            number *= 10;
            number += {
                match num {
                    "1" | "one"   => 1,
                    "2" | "two"   => 2,
                    "3" | "three" => 3,
                    "4" | "four"  => 4,
                    "5" | "five"  => 5,
                    "6" | "six"   => 6,
                    "7" | "seven" => 7,
                    "8" | "eight" => 8,
                    "9" | "nine"  => 9,
                    _ => unreachable!(),
                }
            };
        }
        total += number;
    }
    println!("Part 2 Total: {}", total);
    println!("Note that this answer will be wrong because the problem has \
              overlapping patterns in the input data that is expected to be \
              found. The regex crate does not support this.");

    Ok(())
}
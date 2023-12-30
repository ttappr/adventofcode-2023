//! Advent of Code, Day 1: Trebuchetf?!
//! Part 1, Get the sum of all calibration values from input data.
//! Part 2, Get the som of all calibration values considering numbers spelled 
//!         out.

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let input1 = "./data/sample1.txt";
    let input2 = "./data/sample2.txt";

    println!("Day 1 of Advent of Code! (Trebuchetf?!)");

    part_1(input1)?;
    part_2(input2)?;
    
    let duration = start.elapsed();

    println!("Time elapsed to complete part 1 & 2 is: {:?}", duration);

    Ok(())
}

/// Part 1, Get the sum of all calibration values from input data.
/// 
fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
    let file   = File::open(path)?;    
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

/// Part 2, Get the some of all calibration values considering numbers spelled
///         out.
/// 
fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
    use std::mem::swap;
    let file     = File::open(path)?;    
    let reader   = BufReader::new(file);
    let numbers  = "|one|two|three|four|five|six|seven|eight|nine\
                    |1|2|3|4|5|6|7|8|9|";
    let bnumbers = numbers.as_bytes();
    let n        = numbers.len();

    let mut total = 0;

    for line in reader.lines() {
        let     line     = line?;
        let mut number   = 0;
        let mut matches1 = vec![usize::MAX; n + 1];
        let mut matches2 = vec![usize::MAX; n + 1];
        let mut first    = None;
        let mut last     = None;

        for b1 in line.bytes().chain([b'#']) {
            for (j, b2) in (1..).zip(numbers.bytes()) {
                if b2 == b'|' && matches1[j - 1] != usize::MAX {
                    let k = matches1[j - 1];
                    if first.is_none() {
                        first = Some(&numbers[k..j - 1]);
                    } else {
                        last = Some(&numbers[k..j - 1]);
                    }
                } else if b1 == b2 {
                    if bnumbers[j - 2] == b'|' {
                        matches2[j] = j - 1;
                    } else {
                        matches2[j] = matches1[j - 1];
                    }
                }
            }
            swap(&mut matches1, &mut matches2);
            matches2.fill(usize::MAX);
        }
        let digit1 = first.ok_or("No first digit found")?;
        let digit2 = last.unwrap_or(digit1);

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

    Ok(())
}

//! Advent of Code, Day 1: Trebuchet?!
//! Part 1, Get the sum of all calibration values from input data.
//! Part 2, Get the som of all calibration values considering numbers spelled 
//!         out.

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    println!("Day 1 of Advent of Code! (Trebuchet?!)");

    part_1("./data/sample1.txt")?;
    part_2("./data/sample2.txt")?;
    
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
    let file   = File::open(path)?;    
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let     line     = line?;
        let mut number   = 0;

        let (first, last) = match_nums(&line);

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

/// Match numbers in a string using a dynamic programming approach, since
/// the Rust regex crate doesn't support lookaheads to sort through overlapping
/// matches.
/// 
fn match_nums(line: &str) -> (Option<&str>, Option<&str>) {
    use std::mem::swap;
    const NUMBERS: &str = "|one|two|three|four|five|six|seven|eight|nine\
                           |1|2|3|4|5|6|7|8|9|";
    let bnumbers  = NUMBERS.as_bytes();
    let n         = NUMBERS.len();
    let mut first = None;
    let mut last  = None;
    let mut dp1   = vec![usize::MAX; n + 1];
    let mut dp2   = vec![usize::MAX; n + 1];

    for b1 in line.bytes().chain([b'#']) {
        for (j, b2) in (1..).zip(NUMBERS.bytes()) {
            if b2 == b'|' && dp1[j - 1] != usize::MAX {
                let k = dp1[j - 1];
                if first.is_none() {
                    first = Some(&NUMBERS[k..j - 1]);
                } else {
                    last = Some(&NUMBERS[k..j - 1]);
                }
            } else if b1 == b2 {
                if bnumbers[j - 2] == b'|' {
                    dp2[j] = j - 1;
                } else {
                    dp2[j] = dp1[j - 1];
                }
            }
        }
        swap(&mut dp1, &mut dp2);
        dp2.fill(usize::MAX);
    }
    (first, last)
}

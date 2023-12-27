//! Advent of Code 2023 Day 4 - Scratchcards
//! part 1: Score the winning numbers.
//! Part 2: Count cards and their compounding won copies.

use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 4 Advent of Code");

    //let input = "./data/input1.txt";
    let input = "./data/sample1.txt";

    let n_cards = part_1(input)?;
    
    part_2(input, n_cards)?;
    
    Ok(())
}

/// Part 1: Score the winning numbers. 
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let rexpr  = Regex::new(r"^Card +\d+: +(.*?) \| (.*)")?;
    let to_vec = |s: &str| s.split_whitespace()
                            .map(|s| s.parse())
                            .collect::<Result<Vec<usize>,_>>();

    let mut total   = 0;
    let mut n_cards = 0;

    for line in reader.lines() {
        let line = line?;
        let caps = rexpr.captures(line.as_str()).ok_or("No match")?;
        let win  = to_vec(&caps[1])?;
        let mine = to_vec(&caps[2])?;

        let mut win_nums = [false; 100];
        let mut value    = 0;

        n_cards += 1;

        for w in win {
            win_nums[w] = true;
        }
        for m in mine {
            if win_nums[m] {
                if value == 0 { value  = 1; }
                else          { value *= 2; }
            }
        }
        total += value;
    }
    println!("Part 1 Total: {}", total);
    Ok(n_cards)
}

/// Part 2: Count cards and their compounding won copies.
/// 
fn part_2(path: &str, n_cards: usize) -> Result<(), Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let rexpr  = Regex::new(r"^Card +\d+: +(.*?) \| (.*)")?;
    let to_vec = |s: &str| s.split_whitespace()
                            .map(|s| s.parse())
                            .collect::<Result<Vec<usize>,_>>();

    let mut cards = vec![1; n_cards];

    for (card_i, line) in (1..).zip(reader.lines()) {
        let line = line?;
        let caps = rexpr.captures(line.as_str()).ok_or("No match")?;
        let win  = to_vec(&caps[1])?;
        let mine = to_vec(&caps[2])?;

        let mut win_nums = [false; 100];
        let mut n_wins   = 0;

        for w in win {
            win_nums[w] = true;
        }
        for m in mine {
            if win_nums[m] {
                n_wins += 1;
            }
        }
        for i in card_i..(card_i + n_wins).min(n_cards) {

            cards[i] += cards[card_i - 1];

        }
    }
    println!("Part 2 Total: {}", cards.iter().sum::<usize>());
    Ok(())
} 


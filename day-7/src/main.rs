//! Advent of Code 2023 Day 7 - Camel Cards
//! 
//! Part 1: Evaluate and sort the hands, then calculate the winnings.
//! Part 2: Evaluate the hands with Jokers wild, then calculate the winnings.

use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// Card ordinal values indexed by card character - b'0'.
//
const CARD_ORDER: [u8; 37] = [
    0,  0, 13, 12, 11, 10,  9,  8, 7,  6,  
    0,  0,  0,  0,  0,  0,  0,  1, 0,  0,  
    0,  0,  0,  0,  0,  0,  4,  2, 0,  0,  
    0,  0,  0,  3,  0,  0,  5,
];

const JACK_IDX: usize = 26;

/// Represents the two parts of the problem.
/// 
#[derive(Debug, PartialEq, Copy, Clone)]
enum Part {
    One,
    Two,
}

/// Represents the various poker-like hands of the Camel game.
/// 
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Hand {
    FiveOfKind  ([u8; 5]),
    FourOfKind  ([u8; 5]),
    FullHouse   ([u8; 5]),
    ThreeOfKind ([u8; 5]),
    TwoPair     ([u8; 5]),
    OnePair     ([u8; 5]),
    HighCard    ([u8; 5]),
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 7 Advent of Code\n");

    //let input = "./data/input1.txt";
    let input = "./data/sample1.txt";
    
    let start = Instant::now();

    solution(input, Part::One)?;
    solution(input, Part::Two)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Evaluate and sort the hands, then calculate the winnings.
/// Part 2: Evaluate the hands with Jokers wild, then calculate the winnings.
/// 
fn solution(path: &str, part: Part) -> Result<(), Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hands = Vec::<(Hand, u64)>::new();
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let hand = get_hand(&line[0..5], part);
        let bet  = line[6..].parse()?;

        hands.push((hand, bet));
    }

    hands.sort_unstable();

    for (i, (_, bet)) in (1..).zip(hands.iter().rev()) {
        total += bet * i;
    }

    println!("Part {:?} Total Winnings: {}", part, total);

    Ok(())
}

/// Given the string, `cards``, evaluate the hand according to the Hands 
/// variants. The best hand `cards` evalutes to is returned.
/// 
fn get_hand(cards: &str, part: Part) -> Hand {
    use {Hand::*, Part::*};

    let mut counts  = [0_u8; 37];
    let mut bcards  = [0_u8;  5];
    let mut idxs    = [0   ;  5];
    let mut n_jacks = 0;
    
    for (i, c) in cards.bytes().enumerate() {
        let idx = (c - b'0') as usize;
        let ord = CARD_ORDER[idx];

        if part == Two && idx == JACK_IDX {
            n_jacks     += 1;
            bcards[i]    = u8::MAX;
        } else {
            counts[idx] += 1;
            bcards[i]    = ord;
            
            if counts[idx] == 1 { idxs[i] = idx; }
        }
    }

    let mut hand = HighCard(bcards);

    for card in idxs {
        let count = counts[card];
        
        match count {
            5 => { hand = FiveOfKind(bcards); break; },
            4 => { hand = FourOfKind(bcards); break; },
            3 => {
                match hand {
                    HighCard(b)    => { hand = ThreeOfKind(b);      },
                    OnePair(b)     => { hand = FullHouse(b); break; },
                    _ => (),
                }
            },
            2 => {
                match hand {
                    HighCard(b)    => { hand = OnePair(b);          },
                    OnePair(b)     => { hand = TwoPair(b);   break; },
                    ThreeOfKind(b) => { hand = FullHouse(b); break; },
                    _ => (),
                }
            },
            _ => (),
        }
    }
    if part == Two {
        match n_jacks {
            5 | 
            4 => { hand = FiveOfKind(bcards); },
            3 => { 
                match hand {
                    HighCard(b)    => { hand = FourOfKind(b);  },
                    OnePair(b)     => { hand = FiveOfKind(b);  },
                    _ => (),
                }
            },
            2 => { 
                match hand {
                    HighCard(b)    => { hand = ThreeOfKind(b); },
                    OnePair(b)     => { hand = FourOfKind(b);  },
                    ThreeOfKind(b) => { hand = FiveOfKind(b);  },
                    _ => (),
                }
            },
            1 => { 
                match hand {
                    HighCard(b)    => { hand = OnePair(b);     },
                    OnePair(b)     => { hand = ThreeOfKind(b); },
                    TwoPair(b)     => { hand = FullHouse(b);   },
                    ThreeOfKind(b) => { hand = FourOfKind(b);  },
                    FourOfKind(b)  => { hand = FiveOfKind(b);  },
                    _ => (),
                }
            },
            _ => (),
        }
    }
    hand
}

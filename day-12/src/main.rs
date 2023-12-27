//! Advent of Code 2023 Day 12 - Hot Springs
//! 
//! Part 1: Count different arrangements of operational and broken springs and
//!         give the total sum.
//! Part 2: Quintuple the map of springs and counts, then do the same thing as
//!         part 1.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::iter::once;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 12 Advent of Code\n"); 

    let input = "./data/input1.txt";
    //let input = "./data/sample1.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Count different arrangements of operational and broken springs and
///         give the total sum.
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let lines  = reader.lines();

    let mut memo  = HashMap::new();
    let mut total = 0;

    for line in lines {
        let     line    = line?;
        let mut split   = line.split(' ');
        let     unknown = split.next().ok_or("Missing 'unknown'!")?.as_bytes();
        let     damaged = split.next().ok_or("Missing 'damaged'!")?
                                      .split(',')
                                      .map(str::parse)
                                      .collect::<Result<Vec<usize>,_>>()?;

        total += recurse(&damaged, unknown, &mut memo);

        memo.clear();
    }
    println!("Part 1 Total: {}", total);
    Ok(total)
}

/// Part 2: Quintuple the map of springs and counts, then do the same thing as
///         part 1.
/// 
fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
    let file   = File::open(path)?;
    let reader = BufReader::new(file);
    let lines  = reader.lines();
    
    let mut memo  = HashMap::new();
    let mut total = 0;

    for line in lines {
        let     line    = line?;
        let mut split   = line.split(' ');
        let mut unknown = split.next().ok_or("Missing 'unknown'!")?
                                      .bytes().collect::<Vec<_>>();
        let mut damaged = split.next().ok_or("Missing 'damaged'!")?
                                      .split(',')
                                      .map(str::parse)
                                      .collect::<Result<Vec<usize>,_>>()?;
        
        let (l1, l2) = (unknown.len(), damaged.len());

        unknown = unknown.into_iter()
                         .chain(once(b'?'))
                         .cycle()
                         .take(5 * (l1 + 1) - 1)
                         .collect::<Vec<_>>();

        damaged = damaged.into_iter()
                         .cycle()
                         .take(5 * l2)
                         .collect::<Vec<_>>();

        total += recurse(&damaged, &unknown, &mut memo);

        memo.clear();
    }
    println!("Part 2 Total: {}", total);
    Ok(total)
}

/// The routine that does the actual counting. It takes an array slice of known
/// continguous damaged springs (`dmg`) and a slice of the damaged map of
/// springs in unknown state (`unk`). Returns the many ways the damage groups
/// can be applied to the ruined map.
/// 
fn recurse(dmg   : &[usize], 
           unk   : &[u8], 
           memo  : &mut HashMap<(usize, usize), usize>) 
    -> usize 
{
    if let Some(&count) = memo.get(&(dmg.len(), unk.len())) {
        count
    } else {
        let mut count  = 0;
        let     space  = dmg.iter().sum::<usize>();
        let     limit  = unk.len() - space;
        let     span   = dmg[0];
        let     ulen   = unk.len();

        for i in 0..=limit {
            if i > 0 && unk[i - 1] == b'#' { break; }

            if unk[i..i + span].iter().all(|&b| b != b'.') {
                if dmg.len() == 1 {
                    if unk[i + span..].iter().all(|&b| b != b'#') {
                        count += 1;
                    }
                } else if (i + span == ulen || unk[i + span] != b'#')
                    && ulen > i + space {
                        count += recurse(&dmg[1..], 
                                         &unk[i + span + 1..], 
                                         memo);
                }
            }
        }
        memo.insert((dmg.len(), unk.len()), count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "./data/sample1.txt";
        assert_eq!(part_1(input).unwrap(), 21);

        let input = "./data/input1.txt";
        assert_eq!(part_1(input).unwrap(), 7407);
    }

    #[test]
    fn test_part_2() {
        let input = "./data/sample1.txt";
        assert_eq!(part_2(input).unwrap(), 525152);
    }

    #[test]
    fn test_recurse_1() {
        let memo = &mut HashMap::new();
        let dmg = [2, 3];
        let unk = b".....";
        assert_eq!(recurse(&dmg, unk, memo), 0);
    }

    #[test]
    fn test_recurse_2() {
        let memo = &mut HashMap::new();
        let dmg = [2, 3];
        let unk = b"##..###";
        assert_eq!(recurse(&dmg, unk, memo), 1);
    }

    #[test]
    fn test_recurse_3() {
        let memo = &mut HashMap::new();
        let dmg = [2, 3, 1];
        let unk = b"##..####..#";
        assert_eq!(recurse(&dmg, unk, memo), 0);
    }

    #[test]
    fn test_recurse_4() {
        let memo = &mut HashMap::new();

        let dmg = [2, 3, 1];
        let unk = b"##..###..##";
        assert_eq!(recurse(&dmg, unk, memo), 0);

        memo.clear();

        let dmg = [2, 3, 1];
        let unk = b"###..###..#";
        assert_eq!(recurse(&dmg, unk, memo), 0);

        memo.clear();

        let dmg = [2, 3, 1];
        let unk = b"##..###..#..???";
        assert_eq!(recurse(&dmg, unk, memo), 1);

        memo.clear();

        let dmg = [2, 3, 1];
        let unk = b"##..###..#..??#";
        assert_eq!(recurse(&dmg, unk, memo), 0);
    }
}

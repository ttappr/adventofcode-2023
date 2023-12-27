//! Advent of Code 2023, Day 3: Gear Ratios
//! Part 1: Sum of all part numbers
//! Part 2: Sum of all gear ratios
//! 

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    part_1()?;
    part_2()?;
    Ok(())
}

/// Represents directions from an index into a row or column of the matrix 
/// formed by the input data.
/// 
enum Direction { Lt, Rt, Up, Dn, At, }

macro_rules! is_digit { ($b:expr) => { $b.is_ascii_digit() } }

/// Part 1 of the puzzle. Returns the sum of all the part numbers extracted from
/// the input data.
/// 
fn part_1() -> Result<(), Box<dyn Error>> {
    use Direction::*;
    let matrix = get_matrix("./data/sample1.txt")?;

    let (m, n) = (matrix.len(), matrix[0].len());

    let mut total = 0;
    let mut seen  = HashSet::new();

    for i in 0..m {
        for j in 0..n {
            if is_symbol(matrix[i][j]) {
                for (dir_i, dir_j) in [(Up, Lt), (Up, At), (Up, Rt),
                                       (At, Lt), /* 🦀 */  (At, Rt),
                                       (Dn, Lt), (Dn, At), (Dn, Rt)] {
                    let opt_i = checked_delta(dir_i, i, m);
                    let opt_j = checked_delta(dir_j, j, n);

                    if let (Some(i2), Some(j2)) = (opt_i, opt_j) {

                        if is_digit!(matrix[i2][j2]) {

                            let (num, k, l) = grab_number(&matrix[i2], j2);
                            
                            if seen.insert((i2, k, l)) {
                                total += num;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Part 1 Total: {}", total);
    Ok(())
}

/// Part 2 of the puzzle. The gear ratios are extracted from the input data
/// and their sum is printed.
/// 
fn part_2() -> Result<(), Box<dyn Error>> {
    use Direction::*;
    let matrix = get_matrix("./data/sample1.txt")?;

    let (m, n) = (matrix.len(), matrix[0].len());

    let mut total = 0;
    let mut seen  = HashSet::new();
    let mut nums  = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == b'*' {
                nums.clear();
                for (dir_i, dir_j) in [(Up, Lt), (Up, At), (Up, Rt),
                                       (At, Lt), /* 🦀 */  (At, Rt),
                                       (Dn, Lt), (Dn, At), (Dn, Rt)] {
                    let opt_i = checked_delta(dir_i, i, m);
                    let opt_j = checked_delta(dir_j, j, n);

                    if let (Some(i2), Some(j2)) = (opt_i, opt_j) {

                        if is_digit!(matrix[i2][j2]) {

                            let (num, k, l) = grab_number(&matrix[i2], j2);
                            
                            if seen.insert((i2, k, l)) {
                                nums.push(num);
                            }
                        }
                    }
                }
                if nums.len() == 2 {
                    total += nums[0] * nums[1];
                }
            }
        }
    }
    println!("Part 2 Total: {}", total);
    Ok(())
}

/// Reads the input file given by `file` and returns it as a matrix of bytes.
/// 
fn get_matrix(file: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let file   = File::open(file)?;    
    let reader = BufReader::new(file);
    let matrix = reader.lines().collect::<Result<Vec<_>, _>>()?
                       .into_iter().map(|s| s.bytes().collect())
                       .collect::<Vec<Vec<_>>>();
    Ok(matrix)
}

/// Given the index in an array, `i`, and the length of the array, `n`,
/// return the index of the next element up, down, left, or right if it exists.
/// Otherwise, return `None`.
/// 
fn checked_delta(dir: Direction, i: usize, n: usize) -> Option<usize> {
    use Direction::*;
    match dir {
        Lt | Up => i.checked_sub(1),
        Rt | Dn => if i < n - 1 {Some(i + 1)} else {None},
        _       => Some(i),
    }
}

/// Return `true` if the given byte is a symbol per the puzzle's definition.
/// 
fn is_symbol(b: u8) -> bool {
    b != b'.' && b < b'0' || b > b'9'
}

/// Given a byte array, `ba`, and an index, `i`, return the number at that
/// index. Bytes to the left and right are scanned to see if they are part of
/// the number. The value of the number represented by the character bytes is
/// returned along with the indices of the first and last bytes of the number.
/// 
fn grab_number(ba: &[u8], i: usize) -> (i32, usize, usize) {
    use std::str::from_utf8;
    let mut j = i;
    while j > 0 && is_digit!(ba[j - 1]) { j -= 1; }
    let mut k = i;
    while k < ba.len() && is_digit!(ba[k]) { k += 1; }
    let num = from_utf8(&ba[j..k]).unwrap().parse::<i32>().unwrap();
    (num, j, k)
}


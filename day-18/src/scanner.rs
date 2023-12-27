//! Advent of Code 2023 Day 18 - Lavaduct Lagoon
//! 
//! Part 1: 
//! Part 2: 

use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

#[derive(Clone, Copy)]
enum ColState {
    Mid(i32),
    End(i32),
    Empty,
}    

pub fn scan<R>(reader: R) -> Result<usize, Box<dyn Error>> 
where 
    R: BufRead
{
    use ColState::*;
    let lines  = reader.lines();
    let expr   = Regex::new(r"([UDLR]) +(\d+) +\(#(\w{5})(\w{1})\)")?;

    let mut horizontals = BTreeSet::new();
    let mut cursor   = (0i32, 0i32); // (row, col), or (i, j)
    let mut up_left  = (0, 0);
    let mut lo_right = (0, 0);

    for line in lines {
        let line = line?;
        let cap  = expr.captures(&line).ok_or("Invalid input")?;
        let dir  = cap[1].as_bytes()[0];
        let dist = cap[2].parse::<i32>()?;

        match dir {
            b'U' => {
                cursor.0 -= dist;
                up_left.0 = up_left.0.min(cursor.0);
            },
            b'D' => {
                cursor.0 += dist;
                lo_right.0 = lo_right.0.max(cursor.0);
            },
            b'L' => {
                let p1 = (cursor.0, cursor.1 - dist);
                let p2 = (cursor.0, cursor.1);
                cursor.1 -= dist;
                up_left.1 = up_left.1.min(cursor.1);

                horizontals.insert((p1, p2));
            },
            b'R' => {
                let p1 = (cursor.0, cursor.1);
                let p2 = (cursor.0, cursor.1 + dist);
                cursor.1 += dist;
                lo_right.1 = lo_right.1.max(cursor.1);

                horizontals.insert((p1, p2));
            },
            _ => return Err("Invalid direction".into()),
        }
    }
    let height = (lo_right.0 - up_left.0) as usize + 1;
    let width  = (lo_right.1 - up_left.1) as usize + 1;

    let mut capacity  = 0;
    let mut col_state = vec![Empty; width];

    let mut iter = horizontals.into_iter().peekable();

    while let Some((mut p1, mut p2)) = iter.next() {
        let r1 = p1.0 - up_left.0;

        loop {
            let (_, c1) = p1;
            let (_, c2) = p2;

            let c1 = (c1 - up_left.1) as usize;
            let c2 = (c2 - up_left.1) as usize;

            for c in c1..=c2 {
                match col_state[c] {
                    Empty => {
                        if c == c1 || c == c2 {
                            col_state[c] = End(r1);
                        } else {
                            col_state[c] = Mid(r1);
                        }
                    },
                    Mid(r2) => {
                        if c == c1 || c == c2 {
                            col_state[c] = End(r1);
                        } else {
                            col_state[c] = Empty;
                        }
                        capacity += r1 - r2 + 1;
                    },
                    End(r2) => {
                        if c == c1 || c == c2 {
                            col_state[c] = End(r1);
                            capacity += r1 - r2;
                        } else {
                            col_state[c] = Empty;
                            capacity += r1 - r2;
                        }
                    },
                }
            }

            if iter.peek().map_or(true, |(p1, _)| p1.0 != r1) { break; }

            (p1, p2) = iter.next().unwrap();
        }
    }
    println!("Part 1) Lava Capacity: {}", capacity);
    Ok(capacity as usize)
}


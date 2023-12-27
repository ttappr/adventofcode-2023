#![allow(clippy::manual_range_contains, clippy::single_element_loop)]

//! Advent of Code 2023 Day 24 - Never Tell Me The Odds
//! 
//! Part 1: Considering only the X and Y axes, check all pairs of hailstones' 
//!         future paths for intersections. How many of these intersections 
//!         occur within the test area?
//! Part 2: Determine the exact position and velocity the rock needs to have at 
//!         time 0 so that it perfectly collides with every hailstone. What do 
//!         you get if you add up the X, Y, and Z coordinates of that initial 
//!         position?

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

use mathru::algebra::linear::matrix::{General, Solve};
use mathru::algebra::linear::vector::Vector;
use mathru::vector;

use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 24 Advent of Code\n"); 

    let start = Instant::now();

    for input in ["./data/sample1.txt", 
                  //"./data/input1.txt"
                  ] {
        part_1(input)?;
        part_2(input)?;
    }

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Considering only the X and Y axes, check all pairs of hailstones' 
///         future paths for intersections. How many of these intersections 
///         occur within the test area?
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let hail_stones = parse_input(path)?;
    let mut intersections = 0;

    for (i, h1) in (1..).zip(&hail_stones) { 
        for h2 in &hail_stones[i..] {
            
            // Matrices are column-wise defined - like Fortran.
            let a = General::new(2, 2, vec![h1.vx,  h1.vy, -h2.vx, -h2.vy]);
            let b = vector![h2.x - h1.x; h2.y - h1.y];

            if let Ok(t) = a.solve(&b) {
                if t[0] >= 0. && t[1] >= 0. {
                    let x1 = h1.x + t[0] * h1.vx;
                    let y1 = h1.y + t[0] * h1.vy;
                    if x1 >= 200000000000000. && x1 <= 400000000000000. &&
                       y1 >= 200000000000000. && y1 <= 400000000000000. {
                        intersections += 1;
                    }
                }
            }
        }
    }
    println!("Part 1 Total intersections: {}", intersections);
    Ok(intersections)
}

/// Part 2: Determine the exact position and velocity the rock needs to have at 
///         time 0 so that it perfectly collides with every hailstone. What do 
///         you get if you add up the X, Y, and Z coordinates of that initial 
///         position?
/// 
fn part_2(path: &str) -> Result<f64, Box<dyn Error>> {
    use SatResult::*;
    let hail   = parse_input(path)?;
    let cfg    = Config::new();
    let ctx    = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let rock_x  = Int::new_const(&ctx, "rock_x");
    let rock_y  = Int::new_const(&ctx, "rock_y");
    let rock_z  = Int::new_const(&ctx, "rock_z");
    let rock_vx = Int::new_const(&ctx, "rock_vx");
    let rock_vy = Int::new_const(&ctx, "rock_vy");
    let rock_vz = Int::new_const(&ctx, "rock_vz");
    let zero    = Int::from_i64(&ctx, 0);

    // The sample size and the skip values are kind of arbitrary. The z3 lib
    // is finicky. I had to play around with these values to get a solution.
    for (i, hail) 
        in (0..15).zip(hail.iter().skip(5).map(|h| h.z3_hail_stone(&ctx))) {

        let t = Int::new_const(&ctx, format!("t{}", i));

        solver.assert(&t.gt(&zero));
        solver.assert(&(&rock_x + &rock_vx * &t)._eq(&(hail.x + hail.vx * &t)));
        solver.assert(&(&rock_y + &rock_vy * &t)._eq(&(hail.y + hail.vy * &t)));
        solver.assert(&(&rock_z + &rock_vz * &t)._eq(&(hail.z + hail.vz * &t)));
    }
    if let (Sat, Some(model)) = (solver.check(), solver.get_model()) {
        println!("{:?}", model);
        let soln = model.eval(&(rock_x + rock_y + rock_z), true).unwrap();

        println!("Part 2: Sum of coordinates: {}", soln);
    } else {
        println!("Part 2: No solution found!");
    }

    Ok(0.)
}

/// A struct to hold the hailstone data in a form that can be used by the Z3.
/// 
#[derive(Debug)]
struct Z3HailStone<'a> {
    x  : Int<'a>,
    y  : Int<'a>,
    z  : Int<'a>,
    vx : Int<'a>,
    vy : Int<'a>,
    vz : Int<'a>,
}

/// Represents a single hailstone with position and directional velocity.
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
struct HailStone {
    x  : f64,
    y  : f64,
    z  : f64,
    vx : f64,
    vy : f64,
    vz : f64,
}
impl HailStone {
    fn new(x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> Self {
        Self { x, y, z, vx: dx, vy: dy, vz: dz }
    }
    /// Converts the HailStone struct into a Z3HailStone struct whose fields
    /// are Z3 compatible.
    /// 
    fn z3_hail_stone<'a>(&self, ctx: &'a Context) -> Z3HailStone<'a> {
        Z3HailStone {
            x  : Int::from_i64(ctx, self.x  as i64),
            y  : Int::from_i64(ctx, self.y  as i64),
            z  : Int::from_i64(ctx, self.z  as i64),
            vx : Int::from_i64(ctx, self.vx as i64),
            vy : Int::from_i64(ctx, self.vy as i64),
            vz : Int::from_i64(ctx, self.vz as i64),
        }
    }
}

/// Reads in the input file specified by path and parses it into a vector of
/// HailStone objects.
///
fn parse_input(path: &str) -> Result<Vec<HailStone>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let expr   = Regex::new(&(r"(\d+), +(\d+), +(\d+) +".to_owned() +
                              r"@ +(-?\d+), +(-?\d+), (-?\d+)"))?;

    let mut hail = Vec::new();

    for line in lines {
        let line = line?;
        let caps = expr.captures(&line).ok_or("No match!")?;
        let c = caps.iter().skip(1).map(|x| x.unwrap().as_str().parse())
                                   .collect::<Result<Vec<f64>, _>>()?;
        hail.push(HailStone::new(c[0], c[1], c[2], c[3], c[4], c[5]));
    }
    Ok(hail)
}

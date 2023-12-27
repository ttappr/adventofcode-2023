//! Advent of Code 2023 Day 16 - The Floor Will Be Lava
//! 
//! Part 1: With the light path starting at (0, 0) and going right, analyze the 
//!         paths of light reflecting througout the grid and calculate how many
//!         tiles end up being energized.
//! Part 2: With a light path starting from every point on the circumference of
//!         the grid directed inward, analyze the paths of light reflecting 
//!         throughout the grid for each of them. Find the path that covers the
//!         most tiles and return the number of tiles it covers.

use std::cell::{RefCell, RefMut, Ref};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 16 Advent of Code\n"); 

    //let input = "./data/input1.txt";
    let input = "./data/sample1.txt";
    
    let start = Instant::now();

    part_1(input)?;
    part_2(input)?;

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: With the light path starting at (0, 0) and going left, analyze the 
///         paths of light reflecting througout the grid and calculate how many
///         tiles end up being energized.
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    use {PathStepResult::*, Direction::*};
    let     grid  = get_grid(path)?;
    let mut paths = VecDeque::new();
    let mut init  = LightPath::new(0, 0, Right);

    init.step(false, &grid);

    paths.push_back(init.clone());

    while let Some(mut path) = paths.pop_front() {
        match path.step(true, &grid) {
            Single(p) => {
                paths.push_back(p);
            },
            Split(p1, p2) => {
                paths.push_back(p1);
                paths.push_back(p2);
            },
            End => (),
        }
    }
    let n_tiles = init.coverage();

    println!("Part 1) Tiles covered: {}", n_tiles);
    
    Ok(n_tiles)
}

/// Part 2: With a light path starting from every point on the circumference of
///         the grid directed inward, analyze the paths of light reflecting 
///         throughout the grid for each of them. Find the path that covers the
///         most tiles and return the number of tiles it covers.
/// 
fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
    use {PathStepResult::*, Direction::*};
    let     grid  = get_grid(path)?;
    let     m     = grid.len();
    let     n     = grid[0].len();
    let mut paths = VecDeque::new();

    let mut enqueue = |row, col, dir| {
        match LightPath::new(row, col, dir).step(false, &grid) {
            Single(p) => {
                paths.push_back(p);
            },
            Split(p1, p2) => {
                paths.push_back(p1);
                paths.push_back(p2);
            },
            End => (),
        }
    };
    for row in 0..m {
        enqueue(row,     0, Right);
        enqueue(row, n - 1, Left);
    }
    for col in 0..n {
        enqueue(    0, col, Down);
        enqueue(m - 1, col, Up);
    }
    let init = paths.clone();

    while let Some(mut path) = paths.pop_front() {
        match path.step(true, &grid) {
            Single(p) => {
                paths.push_back(p);
            },
            Split(p1, p2) => {
                paths.push_back(p1);
                paths.push_back(p2);
            },
            End => (),
        }
    }
    let n_tiles = init.iter().map(|path| path.coverage()).max().unwrap();

    println!("Part 2) Tiles covered: {}", n_tiles);

    Ok(n_tiles)
}

/// Load and return the grid from the input file.
/// 
fn get_grid(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let grid   = lines.map(|line| Ok(line?.into_bytes()))
                      .collect::<Result<Vec<Vec<u8>>,Box<dyn Error>>>()?;
    Ok(grid)
}

/// Represents a set of visited tiles. HashSet wrapped in smart pointers.
/// 
type Visited = Rc<RefCell<HashSet<(usize, usize, Direction, bool)>>>;

/// Represents the directions LightPath's can travel.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction { Up, Down, Left, Right, }

/// Represents the result of a LightPath's step.
/// 
enum PathStepResult {
    Single(LightPath),
    Split(LightPath, LightPath),
    End,
}

/// Represents a path of light traveling through the grid.
/// 
#[derive(Debug, Clone)]
struct LightPath {
    row  : usize,
    col  : usize,
    dir  : Direction,
    seen : Visited,
}
impl LightPath {
    fn new(row: usize, col: usize, dir: Direction) -> Self {
        Self { row, col, dir, seen: Rc::new(RefCell::new(HashSet::new())) }
    }
    fn split(&self, row: usize, col: usize, dir: Direction) -> Self {
        Self { row, col, dir, seen: self.seen.clone() }
    }
    fn seen(&self) -> Ref<HashSet<(usize, usize, Direction, bool)>> {
        self.seen.borrow()
    }
    fn seen_mut(&self) -> RefMut<HashSet<(usize, usize, Direction, bool)>> {
        self.seen.borrow_mut()
    }
    fn coverage(&self) -> usize {
        self.seen().iter()
            .map(|(row, col, _, _)| (row, col))
            .collect::<HashSet<_>>()
            .len()
    }
   fn step(&mut self, 
            incr : bool,
            grid : &Vec<Vec<u8>>) 
        -> PathStepResult 
    {
        use {Direction::*, PathStepResult::*};
        let (m, n) = (grid.len(), grid[0].len());
        let tuple  = (self.row, self.col, self.dir, incr);

        if self.seen_mut().insert(tuple) {
            let on_grid = {
                match self.dir {
                    Up    => self.row != 0,
                    Down  => self.row != m - 1,
                    Left  => self.col != 0,
                    Right => self.col != n - 1,
                }
            };
            if on_grid {
                if incr {
                    match self.dir {
                        Up    => self.row -= 1,
                        Down  => self.row += 1,
                        Left  => self.col -= 1,
                        Right => self.col += 1,
                    }
                }
                match grid[self.row][self.col] {
                    b'/'  => {
                        match self.dir {
                            Up    => self.dir = Right,
                            Down  => self.dir = Left,
                            Left  => self.dir = Down,
                            Right => self.dir = Up,
                        }
                        Single(self.clone())
                    },
                    b'\\' => {
                        match self.dir {
                            Up    => self.dir = Left,
                            Down  => self.dir = Right,
                            Left  => self.dir = Up,
                            Right => self.dir = Down,
                        }
                        Single(self.clone())
                    },
                    b'-'  => {
                        match self.dir {
                            Up    |
                            Down  => {
                                Split(self.split(self.row, self.col, Left),
                                      self.split(self.row, self.col, Right))
                            },
                            Left  |
                            Right => Single(self.clone()),
                        }
                    },
                    b'|' => {
                        match self.dir {
                            Up    |
                            Down  => Single(self.clone()),
                            Left  |
                            Right => {
                                Split(self.split(self.row, self.col, Up),
                                      self.split(self.row, self.col, Down))
                            },
                        }
                    },
                    _  => Single(self.clone()),
                }
            } else { End }
        } else { End }
    }
}


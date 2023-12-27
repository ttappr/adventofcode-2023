//! Advent of Code 2023 Day 25 - Snowverload
//! 
//! Part 1: Find the three wires you need to disconnect in order to divide the 
//!         components into two separate groups. What do you get if you multiply
//!         the sizes of these two groups together?
//! Part 2: STILL WORKING ON THIS PART.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;

type RwxResult<T> = Result<Option<(usize, Vec<T>)>, Box<dyn Error>>;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 24 Advent of Code\n"); 

    let start = Instant::now();

    for input in ["./data/sample1.txt", 
                  "./data/sample2.txt",
                  //"./data/input1.txt"
                  ] {
        part_1(input)?;
    }

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Find the three wires you need to disconnect in order to divide the 
///         components into two separate groups. What do you get if you multiply
///         the sizes of these two groups together?
/// 
fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
    let     edges = parse_input(path)?;
    let mut graph = UnGraph::new_undirected();

    let verts = edges.iter().flatten().map(|s| s.as_str())
                            .collect::<HashSet<_>>();
    let nodes = verts.iter().map(|&s| (s, graph.add_node(s)))
                            .collect::<HashMap<_, _>>();

    for adjacent in &edges {
        let v1 = adjacent[0].as_str();

        for v2 in adjacent[1..].iter().map(|s| s.as_str()) {

            graph.add_edge(nodes[v1], nodes[v2], 1);
        }
    }
    let min_cut: RwxResult<_> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    if let Ok(Some((_, cut))) = &min_cut {
        let product = (verts.len() - cut.len()) * cut.len();

        println!("Part 1 Product of Subgraph Sizes: {}", product);
    } else {
        println!("Unable to find min cut!");
    }
    Ok(0)
}

/// Reads in the input file specified by path and parses it into a vector of
/// String vectors.
///
fn parse_input(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    lines.map(|l| Ok(l?.split(&[':',' '])
                       .filter(|s| !s.is_empty())
                       .map(|s| s.to_string()).collect())).collect()
}

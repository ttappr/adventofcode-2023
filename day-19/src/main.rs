//! Advent of Code 2023 Day 19 - Aplenty
//! 
//! Part 1: Sort through all of the parts you've been given; what do you get if 
//!         you add together all of the rating numbers for all of the parts that
//!         ultimately get accepted?
//! Part 2: Consider only your list of workflows; the list of part ratings that 
//!         the Elves wanted you to sort is no longer relevant. How many 
//!         distinct combinations of ratings will be accepted by the Elves' 
//!         workflows?

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 19 Advent of Code\n"); 

    let start = Instant::now();

    for input in ["./data/sample1.txt", "./data/input1.txt"] {
        part_1(input)?;
        part_2(input)?;
    }

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Sort through all of the parts you've been given; what do you get if 
///         you add together all of the rating numbers for all of the parts that
///         ultimately get accepted?
///
fn part_1(path: &str) -> Result<i64, Box<dyn Error>> {
    use MatchResult::*;
    let (wflows, parts) = parse_input(path)?;
    let mut total = 0;

    'parts: for part in &parts {
        let mut flow = wflows.get("in").unwrap();
        loop {
            match flow.match_rules(part) {
                Accept => {
                    total += part.total_vals();
                    continue 'parts;
                },
                Reject => {
                    continue 'parts;
                },
                Forward(name) => {
                    flow = wflows.get(&name).unwrap();
                },
            }
        }
    }
    println!("Part 1 Sum of All Ratings........: {}", total);
    Ok(0)
}

/// Part 2: Consider only your list of workflows; the list of part ratings that 
///         the Elves wanted you to sort is no longer relevant. How many 
///         distinct combinations of ratings will be accepted by the Elves' 
///         workflows?
///
fn part_2(path: &str) -> Result<i64, Box<dyn Error>> {
    use RangeResult::*;
    let (wflows, _) = parse_input(path)?;
    let mut stack   = vec![Forward("in".into(), RangePart::new(1, 4000))];
    let mut combos  = 0;

    while let Some(result) = stack.pop() {
        match result {
            Accept(part) => {
                combos += part.num_combos();
            },
            Forward(name, part) => {
                let flow = wflows.get(&name).unwrap();
                for result in flow.range_match_rules(part) {
                    stack.push(result);
                }
            },
        }
    }
    println!("Part 2 Number of Combinations....: {}", combos);
    Ok(0)
}

/// Outcomes of matching a part against a workflow.
/// 
#[derive(Debug)]
enum MatchResult {
    Accept,
    Reject,
    Forward(String),
}

/// Outcomes of matching a range test part against a workflow.
/// 
#[derive(Debug)]
enum RangeResult {
    Accept(RangePart),
    Forward(String, RangePart),
}

/// Represents a series of tests performed against a part producing 
/// MatchResult's.
/// 
#[derive(Debug)]
struct WorkFlow {
    _name   : String,
    default : String,
    rules   : Vec<Rule>,
}
impl WorkFlow {
    fn new(name: &str, default: &str) -> Self {
        Self { _name: name.into(), default: default.into(), rules: Vec::new() }
    }
    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
    /// Matches a part against the workflow, producing a MatchResult which could
    /// be Accept, Reject, or Forward.
    /// 
    fn match_rules(&self, part: &Part) -> MatchResult {
        use MatchResult::*;
        for rule in &self.rules {
            if rule.check_part(part) {
                match rule.dest() {
                    "A" => return Accept,
                    "R" => return Reject,
                    _   => return Forward(rule.dest.clone()),
                }
            }
        }
        match self.default.as_str() {
            "A" => Accept,
            "R" => Reject,
            _   => Forward(self.default.clone()),
        }
    }
    /// Given a part representing ranges of input values, produces a vector
    /// containing of range parts representing all the possible outcomes of 
    /// matching the the ranges against the workflow.
    /// 
    fn range_match_rules(&self, part: RangePart) -> Vec<RangeResult> {
        use RangeResult::*;
        let mut results = Vec::new();
        
        let mut rejpart = part;
        let mut accpart = part;

        for rule in &self.rules {
            if rule.op() == ">" {
                accpart.set_min(&rule.var, rule.val() + 1);
                rejpart.set_max(&rule.var, rule.val());
            } else {
                accpart.set_max(&rule.var, rule.val() - 1);
                rejpart.set_min(&rule.var, rule.val());
            }
            match rule.dest.as_str() {
                "A" => results.push(Accept(accpart)),
                "R" => (), //Reject,
                _   => results.push(Forward(rule.dest.clone(), accpart)),
            }
            accpart = rejpart;
        }
        match self.default.as_str() {
            "A" => results.push(Accept(rejpart)),
            "R" => (), //Reject,
            _   => results.push(Forward(self.default.clone(), rejpart)),
        }
        results
    }
}

/// Represents a single test performed against a part.
/// 
#[derive(Debug)]
struct Rule {
    var  : String,
    dest : String,
    op   : String,
    val  : i64,
}
impl Rule {
    fn new(var: &str, dest: &str, op: &str, val: i64) -> Self {
        Self { var: var.into(), dest: dest.into(), op: op.into(), val }
    }
    fn op(&self) -> &str {
        self.op.as_str()
    }
    fn dest(&self) -> &str {
        self.dest.as_str()
    }
    fn val(&self) -> i64 {
        self.val
    }
    fn check_part(&self, part: &Part) -> bool {
        let val = part.get_val(&self.var);
           self.op() == ">" && val > self.val() 
        || self.op() == "<" && val < self.val()
    }
}

/// Represents a part with attributes x, m, a, and s.
/// 
#[derive(Debug)]
struct Part {
    x : i64,
    m : i64,
    a : i64,
    s : i64,
}
impl Part {
    fn new() -> Self {
        Self { x: 0, m: 0, a: 0, s: 0 }
    }
    fn get_val(&self, attr: &str) -> i64 {
        match attr {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _   => unreachable!(),
        }
    }
    fn set_val(&mut self, attr: &str, val: i64) {
        match attr {
            "x" => self.x = val,
            "m" => self.m = val,
            "a" => self.a = val,
            "s" => self.s = val,
            _   => unreachable!(),
        }
    }
    fn total_vals(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

/// A part that represents all the possible ranges of its attributes: x, m, a, 
/// and s.
/// 
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct RangePart {
    x : (i64, i64), // (min, max)
    m : (i64, i64),
    a : (i64, i64),
    s : (i64, i64),
}
impl RangePart {
    fn new(min: i64, max: i64) -> Self {
        Self { x: (min, max), m: (min, max), a: (min, max), s: (min, max) }
    }
    fn num_combos(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) 
      * (self.m.1 - self.m.0 + 1) 
      * (self.a.1 - self.a.0 + 1) 
      * (self.s.1 - self.s.0 + 1)
    }
    /// Sets the minimum value of the attribute. Returns true the change to the
    /// range didn't invalidate the part.
    /// 
    fn set_min(&mut self, attr: &str, val: i64) -> bool {
        let mval = self.get_val_mut(attr);
        mval.0 = val;
        mval.0 <= mval.1
    }
    /// Sets the maximum value of the attribute. Returns true the change to the
    /// range didn't invalidate the part.
    /// 
    fn set_max(&mut self, attr: &str, val: i64) -> bool {
        let mval = self.get_val_mut(attr);
        mval.1 = val;
        mval.0 <= mval.1
    }
    fn get_val_mut(&mut self, attr: &str) -> &mut (i64, i64) {
        match attr {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _   => unreachable!(),
        }
    }
}

/// Reads in the input file specified by path and parses it into a vector of
/// parts and a vector of workflows.
/// 
fn parse_input(path: &str) 
    -> Result<(HashMap<String, WorkFlow>, Vec<Part>), Box<dyn Error>> 
{
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let expr1  = Regex::new(r"^(\w+)\{(.*?),(\w+)\}$")?;
    let expr2  = Regex::new(r"(\w+)([<>])(\d+):(\w+),?")?;
    let expr3  = Regex::new(r"(\w+)=(\d+)")?;

    let mut wflows = HashMap::new();
    let mut parts  = Vec::new();
    let mut first  = true;

    for line in lines {
        let line = line?;
        if line.is_empty() { first = false; continue; }
        if first {
            let caps    = expr1.captures(&line).ok_or("No match")?;
            let name    = caps[1].to_string();
            let default = caps[3].to_string();

            let mut wflow = WorkFlow::new(&name, &default);
            
            for (_, [var, cmp, val, dest]) in expr2.captures_iter(&caps[2])
                                                   .map(|c| c.extract()) {
                wflow.add_rule(Rule::new(var, dest, cmp, val.parse::<i64>()?));
            }
            wflows.insert(name.clone(), wflow);
        } else {
            let mut part = Part::new();

            for (_, [var, val]) in expr3.captures_iter(&line)
                                        .map(|c| c.extract()) {
                part.set_val(var, val.parse::<i64>()?);
            }
            parts.push(part);
        }
    }
    Ok((wflows, parts))
}

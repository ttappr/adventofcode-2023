//! Advent of Code 2023 Day 20 - Pulse Propagation
//! 
//! Part 1: Consult your module configuration; determine the number of low 
//!         pulses and high pulses that would be sent after pushing the button 
//!         1000 times, waiting for all pulses to be fully handled after each 
//!         push of the button. What do you get if you multiply the total number 
//!         of low pulses sent by the total number of high pulses sent?
//! Part 2: Reset all modules to their default states. Waiting for all pulses 
//!         to be fully handled after each button press, what is the fewest 
//!         number of button presses required to deliver a single low pulse to 
//!         the module named rx?

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 20 Advent of Code\n"); 

    let start = Instant::now();

    for input in ["./data/sample1.txt", 
                  "./data/sample2.txt", 
                  "./data/input1.txt"] {
        part_1(input)?;
        part_2(input)?;
    }

    let duration = start.elapsed();

    println!("\nTime elapsed to complete part 1 & 2 is: {:?}\n", duration);
    
    Ok(())
}

/// Part 1: Consult your module configuration; determine the number of low 
///         pulses and high pulses that would be sent after pushing the button 
///         1000 times, waiting for all pulses to be fully handled after each 
///         push of the button. What do you get if you multiply the total number 
///         of low pulses sent by the total number of high pulses sent?
/// 
fn part_1(path: &str) -> Result<i64, Box<dyn Error>> {
    let mut modules  = parse_input(path)?; 
    let mut mediator = Mediator::new();
    let mut button   = Module::new_button("button", "broadcaster");

    for _ in 0..1000 {
        button.send_pulse(&mut mediator);
        mediator.loop_until_done(&mut modules);
    }

    let counts = mediator.get_pulse_counts();
    let total  = counts.0 * counts.1;

    println!("Part 1 Total: {}", total);
    Ok(total)
}

/// Part 2: Reset all modules to their default states. Waiting for all pulses 
///         to be fully handled after each button press, what is the fewest 
///         number of button presses required to deliver a single low pulse to 
///         the module named rx?
/// 
fn part_2(path: &str) -> Result<i64, Box<dyn Error>> {
    let mut modules   = parse_input(path)?; 
    let mut mediator  = Mediator::new();
    let mut button    = Module::new_button("button", "broadcaster");

    // ql is the only input to rx, below are the inputs to ql.
    let     ql_inputs = ["fh", "ss", "mf", "fz"]; 
    let mut ql_counts = [0; 4];

    for press_n in 1..10000 {
        button.send_pulse(&mut mediator);
        mediator.loop_until_done(&mut modules);

        // Check the inputs to see if any sent a high pulse. If so capture the
        // number of presses it took.
        for (ql_in, ql_c) 
            in ql_inputs.iter().cloned().zip(ql_counts.iter_mut()) {
                if let Some(module) = modules.get(ql_in) {
                    if module.counts().0 > 0 && ql_c == &0 {
                        *ql_c = press_n;
                    }
                }
        }
        if ql_counts.iter().all(|d| *d > 0) {
            break;
        }
    }
    let total = ql_counts.iter().product::<i64>();

    println!("Part 2 Total: {:?}", total);
    Ok(total)
}

/// Returns the inputs that `target` receives pulses from. Useful for analyzing
/// the inputs to modules for part 2.
/// 
#[allow(dead_code)]
fn get_inputs(target: &str, modules: &HashMap<String, Module>) -> Vec<String> {
    let target = target.to_string();
    modules.iter().filter(|(_, m)| m.receivers.contains(&target))
                   .map(|(k, _)| k.clone())
                   .collect::<Vec<String>>()
}

/// Reads in the input file specified by path and parses it into a HashMap
/// holding the modules. The module identifiers are the keys.
///
fn parse_input(path: &str) 
    -> Result<HashMap<String, Module>, Box<dyn Error>> 
{
    use ModuleRole::*;
    let reader = BufReader::new(File::open(path)?);
    let lines  = reader.lines();
    let expr1  = Regex::new(r"([%&]?)(\w+) +-> +(.*)")?;
    let expr2  = Regex::new(r"(\w+)")?;

    let mut modules  = HashMap::<String, Module>::new();

    for line in lines {
        let line = line?;
        let caps = expr1.captures(&line).ok_or("No match")?;
        let (_, [pfx, name, targets]) = caps.extract();
        let targets = expr2.captures_iter(targets).map(|c| c[1].into())
                                                  .collect::<Vec<String>>();
        let module = {
            match pfx {
                "&" => Module::new_conjunction(name, targets),
                "%" => Module::new_flip_flop(name, targets),
                ""  => Module::new_broadcast(name, targets),
                _   => return Err("Invalid prefix".into()),
            }
        };
        modules.insert(name.into(), module);
    }
    // Go through the modules and add the Conjunctions' senders to their maps
    // used to determine what sort of signal they send.
    for (sender_id, module) in modules.clone().iter() {
        for receiver_id in &module.receivers {
            if let Some(receiver) = modules.get_mut(receiver_id) {
                if let Conjunction { ref mut recent } = receiver.role {
                    recent.insert(sender_id.into(), Pulse::Low);
                }
            }
        }
    }
    Ok(modules)
}

/// The module objects are identified by string values rather than pointers.
/// 
type Identifier = String;

/// Represents a pulse sent or received by modules.
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

/// Modules are very similar with some minor differences. The `ModuleRole` enum
/// implements a sort of "decorator" pattern where the variants differentiate
/// the different types of module.
/// 
#[derive(Debug, Clone)]
enum ModuleRole {
    Conjunction { recent: HashMap<Identifier, Pulse> },
    FlipFlop    { recent: Pulse, is_on: bool },
    Broadcast   { recent: Pulse },
    Button,
}

/// Represents one of the modules that connects to others through the mediator.
/// 
#[derive(Debug, Clone)]
struct Module {
    identifier : Identifier,
    role       : ModuleRole,
    receivers  : Vec<String>,
    n_high     : i64,
    n_low      : i64,
}
impl Module {
    fn new(identifier: &str, role: ModuleRole, receivers: Vec<String>) -> Self {
        Self {
            identifier : identifier.into(),
            receivers,
            role,
            n_high : 0,
            n_low  : 0,
        }
    }
    fn new_button(identifier: &str, receiver: &str) -> Self {
        Self::new(identifier, 
                  ModuleRole::Button, 
                  vec![receiver.into()])
    }
    fn new_conjunction(identifier: &str, targets: Vec<Identifier>) -> Self {
        Self::new(identifier, 
                  ModuleRole::Conjunction { recent: HashMap::new() }, 
                  targets)
    }
    fn new_flip_flop(identifier: &str, targets: Vec<Identifier>) -> Self {
        Self::new(identifier, 
                  ModuleRole::FlipFlop { recent: Pulse::Low, is_on: false }, 
                  targets)
    }
    fn new_broadcast(identifier: &str, targets: Vec<Identifier>) -> Self {
        Self::new(identifier, 
                  ModuleRole::Broadcast { recent: Pulse::Low }, 
                  targets)
    }
    /// Returns the pulse counts (high, low) for the module.
    /// 
    fn counts(&self) -> (i64, i64) {
        (self.n_high, self.n_low)
    }
    /// This is called on Modules by the mediator when it delivers messages to
    /// a module from its senders.
    /// 
    fn recv_pulse(&mut self, source: &str, pulse: Pulse) {
        use ModuleRole::*;
        match self.role {
            Conjunction { ref mut recent } => {
                if let Some(input) = recent.get_mut(source) {
                    *input = pulse;
                }
            },
            FlipFlop { ref mut recent, .. } => {
                *recent = pulse;
            },
            Broadcast { ref mut recent } => {
                *recent = pulse;
            },
            Button => { /* not a receiver */ },
        }
    }
    /// This is invoked by the mediator to get the modules to send their pulses
    /// so the mediator can queue them up for delivery.
    /// 
    fn send_pulse(&mut self, mediator: &mut Mediator) {
        use {ModuleRole::*, Pulse::*};
        match self.role {
            Conjunction { ref recent } => {
                let pulse = if recent.values().all(|p| *p == High) 
                            { Low } else { High };
                if pulse == High { self.n_high += 1; }
                else             { self.n_low  += 1; }
                for target in &self.receivers {
                    mediator.send(&self.identifier, target, pulse);
                }
            },
            FlipFlop { recent, ref mut is_on } => {
                if recent == Low {
                    *is_on = !*is_on;
                    let pulse = if *is_on { High } else { Low };
                    if pulse == High { self.n_high += 1; }
                    else             { self.n_low  += 1; }
                    for target in &self.receivers {
                        mediator.send(&self.identifier, target, pulse);
                    }
                }
            },
            Broadcast { recent } => {
                if recent == High { self.n_high += 1; }
                else              { self.n_low  += 1; }
                for target in &self.receivers {
                    mediator.send(&self.identifier, target, recent);
                }
            },
            Button => {
                self.n_low  += 1;
                for target in &self.receivers {
                    mediator.send(&self.identifier, target, Low);
                }
            },
        }
    }
}

/// The mediator is responsible for delivering pulses from modules to their
/// receivers. It also keeps track of the number of high and low pulses sent.
/// 
struct Mediator {
    queue      : VecDeque<(Identifier, Identifier, Pulse)>,
    n_hi_pulse : i64,
    n_lo_pulse : i64,
}
impl Mediator {
    fn new() -> Self {
        Self {
            queue       : VecDeque::new(),
            n_hi_pulse  : 0,
            n_lo_pulse  : 0,
        }
    }
    /// Returns the number of high and low pulses sent.
    /// 
    fn get_pulse_counts(&self) -> (i64, i64) {
        (self.n_hi_pulse, self.n_lo_pulse)
    }
    /// The modules invoke this method to send a pulse throug the mediator
    /// to their receivers.
    /// 
    fn send(&mut self, source: &str, target: &str, pulse: Pulse) {
       // println!("{} -{:?} -> {:?}", source, pulse, target);
        if pulse == Pulse::High 
             { self.n_hi_pulse += 1; }
        else { self.n_lo_pulse += 1; }
        self.queue.push_back((source.into(), target.into(), pulse));
    }
    /// This is called to allow the mediator to loop through its queue
    /// collecting pulse messages from the modules and delivering them to
    /// their receivers.
    /// 
    fn loop_until_done(&mut self, modules: &mut HashMap<Identifier, Module>) {
        while let Some((source, target, pulse)) = self.queue.pop_front() {
            if let Some(module) = modules.get_mut(&target) {
                module.recv_pulse(&source, pulse);
                module.send_pulse(self);
            }
        }
    }
}


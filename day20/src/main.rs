use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

static EXAMPLE_INPUT: &str = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 20 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    solve(input.trim());
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop {
        id: String,
        destinations: Vec<String>,
        on: bool,
    },
    Conjunction {
        id: String,
        destinations: Vec<String>,
        inputs: Vec<(String, bool)>,
    },
    Broadcaster {
        id: String,
        destinations: Vec<String>,
    },
}

impl Module {
    fn process(&mut self, from: &str, high: bool) -> Option<bool> {
        match self {
            Module::FlipFlop { on, .. } => {
                if !high {
                    *on = !*on;
                    Some(*on)
                } else {
                    None
                }
            }
            Module::Conjunction { inputs, .. } => {
                let mut all_high = true;
                for (id, last_high) in inputs {
                    if id == from {
                        *last_high = high;
                    } else if !*last_high {
                        all_high = false;
                    }
                }

                if all_high {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            Module::Broadcaster { .. } => Some(high),
        }
    }
}

impl Module {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((parse_flip_flop, parse_conjunction, parse_broadcaster))(i)
    }
}

fn parse_flip_flop(i: &str) -> IResult<&str, Module> {
    map(
        separated_pair(
            preceded(tag("%"), parse_id),
            tag(" -> "),
            separated_list1(tag(","), parse_id),
        ),
        |(id, destinations)| Module::FlipFlop {
            id: id.to_string(),
            destinations: destinations.iter().map(|s| s.to_string()).collect(),
            on: false,
        },
    )(i)
}

fn parse_conjunction(i: &str) -> IResult<&str, Module> {
    map(
        separated_pair(
            preceded(tag("&"), parse_id),
            tag(" -> "),
            separated_list1(tag(","), parse_id),
        ),
        |(id, destinations)| Module::Conjunction {
            id: id.to_string(),
            destinations: destinations.iter().map(|s| s.to_string()).collect(),
            inputs: vec![],
        },
    )(i)
}

fn parse_broadcaster(i: &str) -> IResult<&str, Module> {
    map(
        separated_pair(
            tag("broadcaster"),
            tag(" -> "),
            separated_list1(tag(","), parse_id),
        ),
        |(_, destinations)| Module::Broadcaster {
            id: "broadcaster".to_string(),
            destinations: destinations.iter().map(|s| s.to_string()).collect(),
        },
    )(i)
}

fn solve(input: &str) {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| Module::parse(line).unwrap().1)
        .map(|module| {
            let id = match &module {
                Module::FlipFlop { id, .. } => id,
                Module::Conjunction { id, .. } => id,
                Module::Broadcaster { id, .. } => id,
            };
            (id.to_string(), module)
        })
        .collect();

    // fill conjunction inputs
    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    for module in modules.values() {
        match module {
            Module::FlipFlop {
                id, destinations, ..
            } => {
                for destination in destinations {
                    let existing = input_map.entry(destination.to_string()).or_default();
                    existing.push(id.to_string());
                }
            }
            Module::Conjunction {
                id, destinations, ..
            } => {
                for destination in destinations {
                    let existing = input_map.entry(destination.to_string()).or_default();
                    existing.push(id.to_string());
                }
            }
            Module::Broadcaster { id, destinations } => {
                for destination in destinations {
                    let existing = input_map.entry(destination.to_string()).or_default();
                    existing.push(id.to_string());
                }
            }
        }
    }

    for module in &mut modules {
        if let (_, Module::Conjunction { id, inputs, .. }) = module {
            if let Some(input_ids) = input_map.get(id) {
                for input_id in input_ids {
                    inputs.push((input_id.to_string(), false));
                }
            }
        }
    }
    println!("Modules: {:#?}", modules);

    let mut n_low = 0;
    let mut n_high = 0;

    for _ in 0..1000 {
        let mut pulse_queue: VecDeque<(String, String, bool)> = VecDeque::new();
        pulse_queue.push_back(("broadcaster".to_string(), "a".to_string(), false));
        // pulse_queue.push_back(("broadcaster".to_string(), "b".to_string(), false));
        // pulse_queue.push_back(("broadcaster".to_string(), "c".to_string(), false));
        // pulse_queue.push_back(("broadcaster".to_string(), "nt".to_string(), false));
        // pulse_queue.push_back(("broadcaster".to_string(), "kx".to_string(), false));
        // pulse_queue.push_back(("broadcaster".to_string(), "rc".to_string(), false));
        // pulse_queue.push_back(("broadcaster".to_string(), "mg".to_string(), false));
        n_low += 1;

        while let Some((from, to, high)) = pulse_queue.pop_front() {
            if let Some(module) = modules.get_mut(to.as_str()) {
                if let Some(high) = module.process(from.as_str(), high) {
                    match module {
                        Module::FlipFlop { destinations, .. } => {
                            for destination in destinations {
                                pulse_queue.push_back((to.clone(), destination.to_string(), high));
                                if high {
                                    n_high += 1;
                                } else {
                                    n_low += 1;
                                }
                            }
                        }
                        Module::Conjunction { destinations, .. } => {
                            for destination in destinations {
                                pulse_queue.push_back((to.clone(), destination.to_string(), high));
                                if high {
                                    n_high += 1;
                                } else {
                                    n_low += 1;
                                }
                            }
                        }
                        Module::Broadcaster { destinations, .. } => {
                            for destination in destinations {
                                pulse_queue.push_back((to.clone(), destination.to_string(), high));
                                if high {
                                    n_high += 1;
                                } else {
                                    n_low += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Low: {}, High: {}", n_low, n_high);
    println!("Result: {}", n_low * n_high);
}

fn parse_id(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

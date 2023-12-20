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
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 20 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    solve(input.trim());
}

#[derive(Debug)]
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
        .map(|module| match module {
            Module::FlipFlop { id, .. } => (id, module),
            Module::Conjunction { id, .. } => (id, module),
            Module::Broadcaster { id, .. } => (id, module),
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

    // println!("Modules: {:#?}", modules);

    let mut pulse_queue: VecDeque<(String, bool)> = VecDeque::new();
    let broadcaster = modules.get("broadcaster").unwrap();

    // for _ in 0..1000 {
    broadcast(broadcaster, &mut pulse_queue);
    while let Some((id, high)) = pulse_queue.pop_front() {
        // println!("Pulse: {} {}", id, on);
        let module = modules.get_mut(&id).unwrap();

        match module {
            Module::FlipFlop {
                mut on,
                destinations,
                ..
            } => {
                if !high {
                    on = !on;
                    destinations.iter().for_each(|id| {
                        pulse_queue.push_back((id.to_string(), on));
                    });
                }
            }
            Module::Conjunction { inputs, .. } => {
                // TODO: need to check from?
                let all_on = inputs.iter().all(|(_, on)| *on);
                if all_on {
                    pulse_queue.push_back((id.to_string(), true));
                }
            }
            Module::Broadcaster { .. } => {
                broadcast(module, &mut pulse_queue);
            }
        }
    }
    // }

    println!("Modules: {:#?}", modules);
}

fn broadcast(broadcast_module: &Module, pulse_queue: &mut VecDeque<(String, bool)>) {
    let Module::Broadcaster { destinations, .. } = broadcast_module else {
        panic!("Expected Broadcaster module");
    };

    for destination in destinations {
        pulse_queue.push_back((destination.to_string(), false));
    }
}

fn parse_id(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

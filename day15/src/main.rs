use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::{separated_pair, terminated},
    IResult,
};

static EXAMPLE_INPUT: &str = r#"
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"#;

fn hash(i: &str) -> usize {
    let mut hash = 0;
    for c in i.chars() {
        let ascii = c as u8;
        hash += ascii as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 15 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let start = std::time::Instant::now();
    part1(input.trim());
    println!("Time: {:?}\n", start.elapsed());
    let start = std::time::Instant::now();
    part2(input.trim());
    println!("Time: {:?}\n", start.elapsed());
}

fn part1(input: &str) {
    let strings: Vec<&str> = input.split(',').collect();

    let sum = strings.iter().map(|s| hash(s)).sum::<usize>();
    println!("Part 1: {}", sum);
}

#[derive(Debug, Clone)]
struct Box {
    id: usize,
    lenses: Vec<Lens>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

enum Action {
    Add(String, usize),
    Remove(String),
}

impl Action {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((Self::parse_add, Self::parse_remove))(i)
    }

    fn parse_add(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(alpha1, tag("="), parse_number),
            |(label, focal_length)| Self::Add(label.to_string(), focal_length),
        )(i)
    }

    fn parse_remove(i: &str) -> IResult<&str, Self> {
        map(terminated(alpha1, tag("-")), |label: &str| {
            Self::Remove(label.to_string())
        })(i)
    }
}

fn part2(input: &str) {
    let strings: Vec<&str> = input.split(',').collect();

    let mut boxes: HashMap<usize, Box> = HashMap::new();

    for string in strings {
        let action = Action::parse(string).unwrap().1;
        let box_number = hash(match &action {
            Action::Add(label, _) => label,
            Action::Remove(label) => label,
        }) % 256;

        match action {
            Action::Add(label, focal_length) => {
                if let Some(box_) = boxes.get_mut(&box_number) {
                    if let Some(i) = box_.lenses.iter().position(|l| l.label == label) {
                        // replace lens
                        box_.lenses[i] = Lens {
                            label,
                            focal_length,
                        };
                    } else {
                        // add lens
                        box_.lenses.push(Lens {
                            label,
                            focal_length,
                        });
                    }
                } else {
                    let new_box = Box {
                        id: box_number,
                        lenses: vec![Lens {
                            label,
                            focal_length,
                        }],
                    };
                    boxes.insert(box_number, new_box);
                }
            }
            Action::Remove(label) => {
                if let Some(box_) = boxes.get_mut(&box_number) {
                    let mut i = 0;
                    while i < box_.lenses.len() {
                        if box_.lenses[i].label == label {
                            box_.lenses.remove(i);
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
    }

    let sum = boxes
        .values()
        .flat_map(|box_| {
            box_.lenses
                .iter()
                .enumerate()
                .map(|(i, lens)| (box_.id + 1) * (i + 1) * lens.focal_length)
        })
        .sum::<usize>();

    println!("Part 2: {}", sum);
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

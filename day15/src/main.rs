use std::collections::HashMap;

use indexmap::IndexMap;
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
    lenses: IndexMap<String, usize>,
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
    let actions = input
        .split(',')
        .map(|s| Action::parse(s).unwrap().1)
        .collect::<Vec<_>>();

    let mut boxes: HashMap<usize, Box> = HashMap::new();
    for action in actions {
        let box_number = hash(match &action {
            Action::Add(label, _) => label,
            Action::Remove(label) => label,
        }) % 256;

        match action {
            Action::Add(label, focal_length) => {
                if let Some(box_) = boxes.get_mut(&box_number) {
                    box_.lenses.insert(label, focal_length);
                } else {
                    let new_box = Box {
                        id: box_number,
                        lenses: IndexMap::from([(label, focal_length)]),
                    };
                    boxes.insert(box_number, new_box);
                }
            }
            Action::Remove(label) => {
                if let Some(box_) = boxes.get_mut(&box_number) {
                    box_.lenses.shift_remove_entry(&label);
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
                .map(|(i, lens)| (box_.id + 1) * (i + 1) * lens.1)
        })
        .sum::<usize>();

    println!("Part 2: {}", sum);
}

fn hash(i: &str) -> usize {
    i.chars()
        .map(|c| c as usize)
        .fold(0, |acc, ascii| (acc + ascii) * 17 % 256)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

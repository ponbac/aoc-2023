use std::collections::HashMap;

use num::integer::lcm;

static EXAMPLE_INPUT: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 8 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let steps = input.lines().next().unwrap();
    for line in input.lines().skip(2) {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        let mut value_parts = value.split(", ");
        let left = value_parts
            .next()
            .unwrap()
            .trim_matches('(')
            .trim_matches(')');
        let right = value_parts
            .next()
            .unwrap()
            .trim_matches('(')
            .trim_matches(')');

        map.insert(key, (left, right));
        // println!("{}: ({}, {})", key, left, right);
    }
    // let start = "AAA";
    // let goal = "ZZZ";

    // println!("{:?}, len {}", steps, steps.len());
    // println!("{:?}", map);
    // println!("{:?}", start);
    // println!("{:?}", goal);

    // let mut current = start;
    // let mut n_steps = 0;
    // while current != goal {
    //     let step = steps.chars().nth(n_steps % steps.len()).unwrap();

    //     current = match step {
    //         'L' => map.get(current).unwrap().0,
    //         'R' => map.get(current).unwrap().1,
    //         _ => panic!("Unknown step: {}", step),
    //     };
    //     n_steps += 1;
    // }

    // println!("Part 1 - Steps: {}", n_steps);

    let paths_ending_with_a = map
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();

    println!("{:?}", paths_ending_with_a);

    let mut cycles = vec![];
    for path in paths_ending_with_a {
        let mut current = path;
        let mut n_steps = 0;
        while !current.ends_with('Z') {
            let step = steps.chars().nth(n_steps % steps.len()).unwrap();

            current = match step {
                'L' => map.get(current).unwrap().0,
                'R' => map.get(current).unwrap().1,
                _ => panic!("Unknown step: {}", step),
            };
            n_steps += 1;
        }

        cycles.push(n_steps);
    }

    let lcm = cycles.iter().fold(1, |acc, x| lcm(acc, *x));
    println!("Part 2 - Steps: {}", lcm);
}

fn part2(input: &str) {
    todo!()
}

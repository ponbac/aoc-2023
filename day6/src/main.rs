use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

static EXAMPLE_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 6 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    // time them
    let now = std::time::Instant::now();
    part1(input);
    println!("Time: {:?}\n", now.elapsed());
    let now = std::time::Instant::now();
    part2(input);
    println!("Time: {:?}", now.elapsed());
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut sum = 1;
    // figure out how many possible ways you can win for each race
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut possible_wins = 0;
        for i in 0..*time {
            let speed = i;
            let time_to_move = *time - i;
            let distance_moved = speed * time_to_move;
            if distance_moved > *distance {
                possible_wins += 1;
            }
        }
        sum *= possible_wins;
    }

    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>()
        .unwrap();

    let possible_wins = (0..time)
        .into_par_iter()
        .map(|i| {
            let speed = i;
            let time_to_move = time - i;

            let distance_moved = speed * time_to_move;
            if distance_moved > distance {
                1
            } else {
                0
            }
        })
        .sum::<u64>();

    println!("Part 2: {}", possible_wins);
}

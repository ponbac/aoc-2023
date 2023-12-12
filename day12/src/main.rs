use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

static EXAMPLE_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 12 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();

    // let start = std::time::Instant::now();
    // part1(input);
    // println!("Time: {:?}\n", start.elapsed());
    part2(input);
}

// fn part1(input: &str) {
//     let lines: Vec<(Vec<u8>, Vec<u8>)> = input
//         .lines()
//         .map(|line| {
//             let mut parts = line.split_whitespace();
//             let pattern = parts.next().unwrap().chars().map(|c| c as u8).collect();
//             let numbers = parts
//                 .next()
//                 .unwrap()
//                 .split(',')
//                 .map(|n| n.parse::<u8>().unwrap())
//                 .collect();
//             (pattern, numbers)
//         })
//         .collect();

//     let arrangement_sums = (0..lines.len())
//         .into_par_iter()
//         .enumerate()
//         .map(|(i, _)| {
//             let (pattern, numbers) = &lines[i];
//             let count = count_arrangements(
//                 pattern, numbers, 0, //, &mut HashSet::new()
//             );
//             println!("{}: {:?}", i, pattern);
//             count
//         })
//         .collect::<Vec<_>>();

//     println!("Part 1: {:?}", arrangement_sums.iter().sum::<usize>());
// }

fn part2(_input: &str) {
    let lines: Vec<(Vec<u8>, Vec<u8>)> = _input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let repeated_pattern_part = repeat(parts.next().unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let pattern = repeated_pattern_part.chars().map(|c| c as u8).collect();

            let repeated_numbers_part = repeat(parts.next().unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join(",");
            let numbers = repeated_numbers_part
                .split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            (pattern, numbers)
        })
        .collect();

    let sum = (0..lines.len())
        .into_par_iter()
        .enumerate()
        .map(|(i, _)| {
            let (pattern, numbers) = &lines[i];
            println!("Starting {}: {:?}", i, pattern);
            let count = count_arrangements(pattern, numbers, 0, &mut HashMap::new());
            println!("Finished {}: {:?}", i, pattern);
            count
        })
        .sum::<usize>();

    println!("Part 2: {:?}", sum);
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct MemoKey {
    pattern: Vec<u8>,
    numbers: Vec<u8>,
    start: usize,
}

fn count_arrangements(
    pattern: &[u8],
    numbers: &[u8],
    start: usize,
    cache: &mut HashMap<MemoKey, usize>,
) -> usize {
    if start >= pattern.len() {
        return if is_valid_arrangement(pattern, numbers) {
            1
        } else {
            0
        };
    }

    if pattern[start] != b'?' {
        return count_arrangements(pattern, numbers, start + 1, cache);
    }

    let key = MemoKey {
        pattern: pattern[..start].to_vec(),
        numbers: numbers.to_vec(),
        start,
    };

    if let Some(&count) = cache.get(&key) {
        return count;
    }

    let mut total = 0;
    let mut new_pattern = pattern.to_vec();
    for &c in &[b'#', b'.'] {
        new_pattern[start] = c;
        if can_be_valid(&new_pattern, numbers, start) {
            total += count_arrangements(&new_pattern, numbers, start + 1, cache);
            cache.insert(
                MemoKey {
                    pattern: new_pattern[..start].to_vec(),
                    numbers: numbers.to_vec(),
                    start: start + 1,
                },
                total,
            );
        }
    }

    total
}

fn is_valid_arrangement(pattern: &[u8], numbers: &[u8]) -> bool {
    let mut counts = vec![];
    let mut current_count = 0;

    for &c in pattern {
        if c == b'#' {
            current_count += 1;
        } else if current_count > 0 {
            counts.push(current_count);
            current_count = 0;
        }
    }

    if current_count > 0 {
        counts.push(current_count);
    }

    counts == numbers
}

fn can_be_valid(pattern: &[u8], numbers: &[u8], upto: usize) -> bool {
    let mut num_counts = vec![0; numbers.len()];
    let mut current_group = 0;
    let mut has_broken_spring = false;

    for &c in &pattern[..=upto] {
        match c {
            b'#' => {
                has_broken_spring = true;
                if current_group < numbers.len() {
                    num_counts[current_group] += 1;
                } else {
                    return false;
                }
            }
            b'.' => {
                if has_broken_spring {
                    has_broken_spring = false;
                    current_group += 1;
                }
            }
            _ => {}
        }
    }

    for (i, &count) in num_counts.iter().enumerate() {
        if i <= current_group {
            if count > numbers[i] {
                return false;
            }
        } else if count > 0 {
            return false;
        }
    }

    true
}

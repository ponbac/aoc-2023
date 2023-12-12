use std::{collections::HashMap, iter::repeat};

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

static EXAMPLE_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 12 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let start = std::time::Instant::now();
    part1(input);
    println!("Time: {:?}\n", start.elapsed());
    let start = std::time::Instant::now();
    part2(input);
    println!("Time: {:?}\n", start.elapsed());
}

fn part1(input: &str) {
    let lines: Vec<(Vec<u8>, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap().chars().map(|c| c as u8).collect();
            let numbers = parts
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (pattern, numbers)
        })
        .collect();

    let arrangements_sum = (0..lines.len())
        .into_par_iter()
        .enumerate()
        .map(|(i, _)| {
            let (pattern, numbers) = &lines[i];
            count_arrangements(pattern, numbers, 0, 0, 0, &mut HashMap::new())
        })
        .sum::<usize>();

    println!("Part 1: {:?}", arrangements_sum);
}

fn part2(_input: &str) {
    let lines: Vec<(Vec<u8>, Vec<usize>)> = _input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let repeated_pattern = repeat(parts.next().unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join("?")
                .chars()
                .map(|c| c as u8)
                .collect::<Vec<_>>();

            let repeated_numbers = repeat(parts.next().unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join(",")
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (repeated_pattern, repeated_numbers)
        })
        .collect();

    let sum = (0..lines.len())
        // .into_par_iter()
        .enumerate()
        .map(|(i, _)| {
            let (pattern, numbers) = &lines[i];
            count_arrangements(pattern, numbers, 0, 0, 0, &mut HashMap::new())
        })
        .sum::<usize>();

    println!("Part 2: {:?}", sum);
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct MemoKey {
    pattern_index: usize,
    number_index: usize,
    damaged_count: usize,
}

fn count_arrangements(
    pattern: &[u8],
    numbers: &[usize],
    pattern_index: usize,
    number_index: usize,
    damaged_count: usize,
    cache: &mut HashMap<MemoKey, usize>,
) -> usize {
    if let Some(&arrangements) = cache.get(&MemoKey {
        pattern_index,
        number_index,
        damaged_count,
    }) {
        return arrangements;
    }

    let mut pattern_index = pattern_index;
    let mut number_index = number_index;
    let mut damaged_count = damaged_count;

    loop {
        // if end of pattern reached
        if pattern_index == pattern.len() {
            let valid_end = (number_index == numbers.len() && damaged_count == 0) // gone past but no damage
                || (number_index == numbers.len() - 1 && damaged_count == numbers[number_index]); // at last number and correct damage

            let result = if valid_end { 1 } else { 0 };
            cache.insert(
                MemoKey {
                    pattern_index,
                    number_index,
                    damaged_count,
                },
                result,
            );
            return result;
        }

        match pattern[pattern_index] {
            b'?' => break,
            b'#' => {
                let invalid_damage =
                    number_index > numbers.len() - 1 || damaged_count > numbers[number_index];

                if invalid_damage {
                    cache.insert(
                        MemoKey {
                            pattern_index,
                            number_index,
                            damaged_count,
                        },
                        0,
                    );
                    return 0;
                }

                damaged_count += 1;
                pattern_index += 1;
            }
            b'.' => {
                if damaged_count == 0 {
                    // group not started yet, skip
                    pattern_index += 1;
                } else if damaged_count == numbers[number_index] {
                    // group finished, look for the next one
                    pattern_index += 1;
                    number_index += 1;
                    damaged_count = 0;
                } else {
                    // invalid end of group
                    cache.insert(
                        MemoKey {
                            pattern_index,
                            number_index,
                            damaged_count,
                        },
                        0,
                    );
                    return 0;
                }
            }
            _ => unreachable!(),
        }
    }

    // reached a '?' in the pattern
    let mut arrangements = 0;

    if damaged_count == 0 {
        if number_index < numbers.len() {
            arrangements +=
                count_arrangements(pattern, numbers, pattern_index + 1, number_index, 1, cache)
                    + count_arrangements(
                        pattern,
                        numbers,
                        pattern_index + 1,
                        number_index,
                        0,
                        cache,
                    );
        } else {
            // no more numbers, only valid if no damage
            arrangements +=
                count_arrangements(pattern, numbers, pattern_index + 1, number_index, 0, cache);
        }
    } else if damaged_count == numbers[number_index] {
        // valid amount of damage, go to next number
        arrangements += count_arrangements(
            pattern,
            numbers,
            pattern_index + 1,
            number_index + 1,
            0,
            cache,
        );
    } else {
        // '?' needs to be damaged
        arrangements += count_arrangements(
            pattern,
            numbers,
            pattern_index + 1,
            number_index,
            damaged_count + 1,
            cache,
        );
    };

    cache.insert(
        MemoKey {
            pattern_index,
            number_index,
            damaged_count,
        },
        arrangements,
    );
    arrangements
}

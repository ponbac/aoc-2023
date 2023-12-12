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
    let lines: Vec<(Vec<u8>, Vec<usize>)> = _input
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
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (pattern, numbers)
        })
        .collect();

    let sum = (0..lines.len())
        // .into_par_iter()
        .enumerate()
        .map(|(i, _)| {
            let (pattern, numbers) = &lines[i];
            let count = count_arrangements(pattern, numbers, 0, 0, 0, &mut HashMap::new());
            println!("{}, finished: {}", i, count);
            count
        })
        .sum::<usize>();

    println!("Part 2: {:?}", sum);
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct MemoKey {
    pattern_index: usize,
    number_index: usize,
    current_count: usize,
}

fn count_arrangements(
    pattern: &[u8],
    numbers: &[usize],
    pattern_index: usize,
    number_index: usize,
    current_count: usize,
    cache: &mut HashMap<MemoKey, usize>,
) -> usize {
    if let Some(&count) = cache.get(&MemoKey {
        pattern_index,
        number_index,
        current_count,
    }) {
        return count;
    }

    let mut pattern_index = pattern_index;
    let mut number_index = number_index;
    let mut current_count = current_count;

    loop {
        if pattern_index >= pattern.len() {
            if (number_index > numbers.len() - 1 && current_count == 0)
                || (number_index == numbers.len() - 1 && current_count == numbers[number_index])
            {
                cache.insert(
                    MemoKey {
                        pattern_index,
                        number_index,
                        current_count,
                    },
                    1,
                );
                return 1;
            }

            cache.insert(
                MemoKey {
                    pattern_index,
                    number_index,
                    current_count,
                },
                0,
            );
            return 0;
        }

        match pattern[pattern_index] {
            b'?' => break,
            b'#' => {
                if number_index > numbers.len() - 1 {
                    cache.insert(
                        MemoKey {
                            pattern_index,
                            number_index,
                            current_count,
                        },
                        0,
                    );
                    return 0;
                }

                if current_count == numbers[number_index] {
                    cache.insert(
                        MemoKey {
                            pattern_index,
                            number_index,
                            current_count,
                        },
                        0,
                    );
                    return 0;
                }

                current_count += 1;
                pattern_index += 1;
            }
            b'.' => {
                if current_count == 0 {
                    pattern_index += 1;
                } else if current_count == numbers[number_index] {
                    pattern_index += 1;
                    current_count = 0;
                    number_index += 1;
                } else {
                    cache.insert(
                        MemoKey {
                            pattern_index,
                            number_index,
                            current_count,
                        },
                        0,
                    );
                    return 0;
                }
            }
            _ => unreachable!(),
        }
    }

    let mut total = 0;

    if current_count == 0 {
        if number_index < numbers.len() {
            total +=
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
            total +=
                count_arrangements(pattern, numbers, pattern_index + 1, number_index, 0, cache);
        }
    } else if number_index > numbers.len() - 1 {
        total += 0;
    } else if current_count == numbers[number_index] {
        total += count_arrangements(
            pattern,
            numbers,
            pattern_index + 1,
            number_index + 1,
            0,
            cache,
        );
    } else {
        total += count_arrangements(
            pattern,
            numbers,
            pattern_index + 1,
            number_index,
            current_count + 1,
            cache,
        );
    }

    cache.insert(
        MemoKey {
            pattern_index,
            number_index,
            current_count,
        },
        total,
    );
    total
}

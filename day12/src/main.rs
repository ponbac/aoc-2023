use std::{collections::HashSet, iter::repeat};

use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

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

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let lines: Vec<(Vec<char>, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let pattern = parts.next().unwrap().chars().collect();
            let numbers = parts
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (pattern, numbers)
        })
        .collect();

    let mut sum = 0;
    for (i, (pattern, numbers)) in lines.iter().enumerate() {
        sum += count_arrangements(pattern, numbers, 0, &mut HashSet::new());
        println!("{}: {:?}", i, pattern);
    }

    println!("Part 1: {:?}", sum);
}

fn part2(_input: &str) {
    let lines: Vec<(Vec<char>, Vec<usize>)> = _input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let repeated_pattern_part = repeat(parts.next().unwrap())
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let pattern = repeated_pattern_part.chars().collect();

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
        .into_par_iter()
        .enumerate()
        .map(|(i, _)| {
            let (pattern, numbers) = &lines[i];
            let count = count_arrangements(pattern, numbers, 0, &mut HashSet::new());
            println!("{}: {:?}", i, pattern);
            count
        })
        .sum::<usize>();

    println!("Part 2: {:?}", sum);
}

fn count_arrangements(
    pattern: &[char],
    numbers: &[usize],
    start: usize,
    cache: &mut HashSet<Vec<char>>,
) -> usize {
    if start >= pattern.len() {
        return if is_valid_arrangement(pattern, numbers) {
            1
        } else {
            0
        };
    }

    if pattern[start] != '?' {
        return count_arrangements(pattern, numbers, start + 1, cache);
    }

    let mut total = 0;
    let mut new_pattern = pattern.to_vec();
    for &c in &['#', '.'] {
        new_pattern[start] = c;
        if !cache.contains(&new_pattern) {
            cache.insert(new_pattern.clone());
            if can_be_valid(&new_pattern, numbers, start) {
                total += count_arrangements(&new_pattern, numbers, start + 1, cache);
            }
        }
    }

    total
}

fn is_valid_arrangement(pattern: &[char], numbers: &[usize]) -> bool {
    let mut counts = vec![];
    let mut current_count = 0;

    for &c in pattern {
        if c == '#' {
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

fn can_be_valid(pattern: &[char], numbers: &[usize], upto: usize) -> bool {
    let mut num_counts = vec![0; numbers.len()];
    let mut current_group = 0;
    let mut has_broken_spring = false;

    for &c in &pattern[..=upto] {
        match c {
            '#' => {
                has_broken_spring = true;
                if current_group < numbers.len() {
                    num_counts[current_group] += 1;
                } else {
                    return false;
                }
            }
            '.' => {
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

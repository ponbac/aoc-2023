use std::collections::VecDeque;

use nom::{character::complete::digit1, combinator::map_res, IResult};

static EXAMPLE_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

fn parse_card(i: &str) -> (Vec<usize>, Vec<usize>) {
    let i = i.split(':').last().unwrap();

    let winning_i = i.split('|').next().unwrap();
    let my_i = i.split('|').last().unwrap();

    let winning = winning_i
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let my = my_i
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (winning, my)
}
// fn parse_cards(i: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
//     i.lines().map(parse_card).collect()
// }

fn main() {
    println!("-- Advent of Code 2023 - Day 4 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let mut cards = vec![];
    for line in input.lines() {
        cards.push(parse_card(line));
    }

    let mut total_cards = 0;
    let mut queue = VecDeque::new();

    for i in 0..cards.len() {
        queue.push_back((i, 1));
    }

    while let Some((card_index, count)) = queue.pop_front() {
        total_cards += count;
        let (ref winning, ref my) = cards[card_index];

        let matches = my.iter().filter(|&&num| winning.contains(&num)).count();

        for next_card_index in card_index + 1..card_index + 1 + matches {
            if next_card_index < cards.len() {
                queue.push_back((next_card_index, count));
            }
        }
    }

    println!("Total cards: {}", total_cards);
}

fn part2(input: &str) {
    todo!()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

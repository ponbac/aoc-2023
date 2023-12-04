use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

static EXAMPLE_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

#[derive(Debug)]
struct Card {
    matches: u32,
}

fn parse_card(i: &str) -> IResult<&str, Card> {
    let i = i.split(": ").nth(1).unwrap().trim();
    let (i, (winning, my)) = separated_pair(
        separated_list1(multispace1, preceded(multispace0, parse_number)),
        tag(" | "),
        separated_list1(multispace1, preceded(multispace0, parse_number)),
    )(i)?;

    Ok((
        i,
        Card {
            matches: my.iter().filter(|&&num| winning.contains(&num)).count() as u32,
        },
    ))
}

fn main() {
    println!("-- Advent of Code 2023 - Day 4 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();

    let sum = cards
        .iter()
        .map(|card| {
            if card.matches > 0 {
                2_u32.pow(card.matches - 1)
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();

    let mut factors = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let factor = factors[i];
        (i..i + card.matches as usize).for_each(|j| factors[j + 1] += factor);
    }

    let sum = factors.iter().sum::<u32>();
    println!("Part 2: {}", sum);
}

fn parse_number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

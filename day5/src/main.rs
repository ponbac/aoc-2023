use nom::{character::complete::digit1, combinator::map_res, IResult};

static EXAMPLE_INPUT: &str = r#"

"#;

fn main() {
    println!("-- Advent of Code 2023 - Day 5 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    todo!()
}

fn part2(input: &str) {
    todo!()
}

fn parse_number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

static EXAMPLE_INPUT: &str = r#"

"#;

fn main() {
    println!("-- Advent of Code 2023 - Day 1 --");

    // let input = include_str!("input.txt");
    let input = EXAMPLE_INPUT;

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    input.len()
}

// cargo watch -x "run -q"

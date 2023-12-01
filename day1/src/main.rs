static EXAMPLE_INPUT: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

// cargo watch -x "run -q"
fn main() {
    println!("-- Advent of Code 2023 - Day 1 --");

    let input = include_str!("input.txt");
    // let input = EXAMPLE_INPUT;

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let first = l.find(|c: char| c.is_numeric()).unwrap();
            let last = l.rfind(|c: char| c.is_numeric()).unwrap();

            let chars = l.chars().collect::<Vec<_>>();
            format!("{}{}", chars[first], chars[last])
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(find_digits)
        .map(|digits| {
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum()
}

fn find_digits(line: &str) -> Vec<usize> {
    let text_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digits = Vec::new();

    let chars = line.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < chars.len() {
        if let Some(digit) = chars[i].to_digit(10) {
            digits.push(digit as usize);
            i += 1;
            continue;
        }

        let mut j = i;
        while j < chars.len() {
            let word = &chars[i..=j];
            let word = word.iter().collect::<String>();
            if let Some(digit) = text_digits.iter().position(|&s| s == word) {
                digits.push(digit);
                i += 1;
                break;
            }
            j += 1;
        }
        i += 1;
    }

    digits
}

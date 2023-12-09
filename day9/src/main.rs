
static EXAMPLE_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 9 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let numbers = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = numbers
        .iter()
        .map(|line| process_line(line.clone()))
        .sum::<i64>();

    println!("Sum: {}", sum);
}

fn process_line(line: Vec<i64>) -> i64 {
    let mut differences = expand_line(vec![line]);

    let diff_len = differences.len();
    for i in 0..diff_len {
        let mut curr = differences[diff_len - (i + 1)].clone();

        if i == 0 {
            curr.insert(0, 0);
            differences[diff_len - (i + 1)] = curr;
            continue;
        }

        let diff_below = differences[diff_len - i].clone();
        let new_value = curr.first().unwrap() - diff_below.first().unwrap();
        curr.insert(0, new_value);
        differences[diff_len - (i + 1)] = curr;
    }

    *differences[0].first().unwrap()
}

fn expand_line(lines: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    if lines
        .last()
        .iter()
        .all(|diffs| diffs.iter().all(|n| *n == 0))
    {
        return lines;
    }

    let current_line = lines.last().unwrap().clone();
    let mut diffs = vec![];

    for i in 0..current_line.len() {
        if i == 0 {
            diffs.push(0);
        } else {
            diffs.push(current_line[i] - current_line[i - 1]);
        }
    }

    let mut new_lines = lines.clone();
    new_lines.push(diffs.iter().skip(1).copied().collect::<Vec<_>>());

    expand_line(new_lines)
}

fn _part2(_input: &str) {
    todo!()
}

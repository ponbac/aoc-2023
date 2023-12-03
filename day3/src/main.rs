use std::collections::HashMap;

use nom::{character::complete::digit1, combinator::map_res, IResult};

static EXAMPLE_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

fn main() {
    println!("-- Advent of Code 2023 - Day 3 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
fn part1(input: &str) {
    // let symbols = vec!['#', '$', '*', '+'];
    let symbols = input
        .lines()
        .flat_map(|l| l.chars())
        .filter(|c| !c.is_ascii_digit() && *c != '.')
        .collect::<Vec<char>>();

    let mut grid = vec![vec![]];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    // remove first row
    grid.remove(0);

    // println!("{:?}", grid[0]);

    // create a map of numbers and their positions, e.g. (0, 0) => 467
    let mut map: HashMap<(usize, usize), (String, usize)> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                // check if digit is part of surrounding number on the same line

                // find index of number start by walking left
                let mut start_index = 0;
                for i in (0..x).rev() {
                    if !grid[y][i].is_ascii_digit() {
                        start_index = i + 1;
                        break;
                    }
                }
                // find index of number end by walking right
                let mut end_index = 0;
                for i in x..row.len() {
                    if !grid[y][i].is_ascii_digit() {
                        end_index = i;
                        break;
                    }
                }

                if end_index == 0 {
                    end_index = row.len();
                }

                println!("row {}: {}..{}", y, start_index, end_index);
                let number = grid[y][start_index..end_index]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                // println!(
                //     "{} is part of number {} ({}..{})",
                //     c, number, start_index, end_index
                // );
                let id = format!("{}-{}-{}", y, start_index, end_index);
                map.insert((y, x), (id, number));
            }
        }
    }

    // for each number check if it has a symbol next to it (even diagonally)
    let mut numbers_with_symbols: Vec<(String, usize, String)> = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                // check if there is a symbol next to it
                let mut has_symbol = false;
                for symbol in &symbols {
                    if y > 1 && grid[y - 1][x] == *symbol
                        || y < grid.len() - 1 && grid[y + 1][x] == *symbol
                        || x > 1 && grid[y][x - 1] == *symbol
                        || x < row.len() - 1 && grid[y][x + 1] == *symbol
                        || y > 1 && x > 1 && grid[y - 1][x - 1] == *symbol
                        || y > 1 && x < row.len() - 1 && grid[y - 1][x + 1] == *symbol
                        || y < grid.len() - 1 && x > 1 && grid[y + 1][x - 1] == *symbol
                        || y < grid.len() - 1 && x < row.len() - 1 && grid[y + 1][x + 1] == *symbol
                    {
                        has_symbol = true;
                        let symbol_pos = if y > 1 && grid[y - 1][x] == *symbol {
                            (y - 1, x)
                        } else if y < grid.len() - 1 && grid[y + 1][x] == *symbol {
                            (y + 1, x)
                        } else if x > 1 && grid[y][x - 1] == *symbol {
                            (y, x - 1)
                        } else if x < row.len() - 1 && grid[y][x + 1] == *symbol {
                            (y, x + 1)
                        } else if y > 1 && x > 1 && grid[y - 1][x - 1] == *symbol {
                            (y - 1, x - 1)
                        } else if y > 1 && x < row.len() - 1 && grid[y - 1][x + 1] == *symbol {
                            (y - 1, x + 1)
                        } else if y < grid.len() - 1 && x > 1 && grid[y + 1][x - 1] == *symbol {
                            (y + 1, x - 1)
                        } else if y < grid.len() - 1
                            && x < row.len() - 1
                            && grid[y + 1][x + 1] == *symbol
                        {
                            (y + 1, x + 1)
                        } else {
                            (0, 0)
                        };
                        let number = map.get(&(y, x)).unwrap();
                        println!(
                            "{} is part of number {} and has a symbol next to it",
                            c, number.1
                        );
                        // if number with id is not already in the list, add it
                        let number_with_symbol_pos = (
                            number.0.clone(),
                            number.1,
                            symbol_pos.0.to_string() + "-" + &symbol_pos.1.to_string(),
                        );
                        if !numbers_with_symbols.contains(&number_with_symbol_pos) && *symbol == '*'
                        {
                            numbers_with_symbols.push(number_with_symbol_pos);
                        }
                        break;
                    }
                }
                if has_symbol {
                    // println!("{} has a symbol next to it", c);
                }
            }
        }
    }

    // sum numbers
    let sum: usize = numbers_with_symbols.iter().map(|n| n.1).sum();
    println!("Sum of numbers with symbols: {}", sum);

    // numbers whose symbol position occurs exactly twice
    let symbol_positions: Vec<String> = numbers_with_symbols
        .iter()
        .map(|n| n.2.clone())
        .collect::<Vec<String>>();
    let mut symbols_with_two_numbers: Vec<String> = symbol_positions
        .iter()
        .filter(|n| symbol_positions.iter().filter(|&x| x == *n).count() == 2)
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    // remove duplicates
    symbols_with_two_numbers.sort();
    symbols_with_two_numbers.dedup();

    println!("Symbols with two numbers: {:?}", symbols_with_two_numbers);
    let mut sum = 0;
    for symbol in symbols_with_two_numbers {
        let numbers = numbers_with_symbols
            .iter()
            .filter(|n| n.2 == symbol)
            // multiply numbers
            .map(|n| n.1)
            .product::<usize>();
        sum += numbers;
    }

    println!("Sum of numbers with symbols: {}", sum);
}

fn part2(input: &str) {
    todo!()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

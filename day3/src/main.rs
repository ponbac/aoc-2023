use itertools::Itertools;

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

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    data: Vec<Vec<char>>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            for c in line {
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

        Self {
            height: grid.len(),
            width: grid[0].len(),
            data: grid,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y)?.get(x).copied()
    }

    fn get_number(&self, x: usize, y: usize) -> Option<(String, usize)> {
        let c = self.get(x, y)?;

        if c.is_ascii_digit() {
            let mut start_idx = x;
            let mut end_idx = x;
            // walk left
            while let Some(c) = self.get(start_idx.saturating_sub(1), y) {
                if c.is_ascii_digit() && start_idx > 0 {
                    start_idx -= 1;
                } else {
                    break;
                }
            }
            // walk right
            while let Some(c) = self.get(end_idx + 1, y) {
                if c.is_ascii_digit() {
                    end_idx += 1;
                } else {
                    break;
                }
            }

            let mut number = String::new();
            for x in start_idx..=end_idx {
                number.push(self.get(x, y)?);
            }

            Some((
                format!("{}-{}-{}", y, start_idx, end_idx),
                number.parse().unwrap(),
            ))
        } else {
            None
        }
    }

    fn get_symbols(&self) -> Vec<GridSymbol> {
        let mut symbols = Vec::new();

        for (y, line) in self.data.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if !c.is_ascii_digit() && *c != '.' {
                    symbols.push(GridSymbol::new(*c, x, y));
                }
            }
        }

        symbols
    }
}

#[derive(Debug)]
struct GridSymbol {
    symbol: char,
    x: usize,
    y: usize,
}

impl GridSymbol {
    fn new(symbol: char, x: usize, y: usize) -> Self {
        Self { symbol, x, y }
    }

    fn surrounding_numbers(&self, grid: &Grid) -> Vec<usize> {
        let mut numbers = Vec::new();

        let start_x = self.x.saturating_sub(1);
        let end_x = if self.x + 1 < grid.width {
            self.x + 1
        } else {
            self.x
        };
        let start_y = self.y.saturating_sub(1);
        let end_y = if self.y + 1 < grid.height {
            self.y + 1
        } else {
            self.y
        };

        for y in start_y..=end_y {
            for x in start_x..=end_x {
                if x == self.x && y == self.y {
                    continue;
                }

                if let Some(c) = grid.get_number(x, y) {
                    numbers.push(c);
                }
            }
        }

        // remove numbers with the same id
        numbers.sort_by(|a, b| a.0.cmp(&b.0));
        numbers.dedup_by(|a, b| a.0 == b.0);
        numbers
            .iter()
            .map(|(_, n)| *n)
            .collect::<Vec<usize>>()
            .into_iter()
            .sorted()
            .collect()
    }
}

fn main() {
    println!("-- Advent of Code 2023 - Day 3 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let grid = Grid::new(input);

    let symbols = grid.get_symbols();

    let all_surrounding_numbers: Vec<usize> = symbols
        .iter()
        .flat_map(|s| s.surrounding_numbers(&grid))
        .collect();

    let sum = all_surrounding_numbers.iter().sum::<usize>();
    println!("Part 1: {}", sum);
}

fn part2(input: &str) {
    let grid = Grid::new(input);

    let symbols = grid.get_symbols();
    let star_symbols_with_two_surrounding_numbers = symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .filter(|s| s.surrounding_numbers(&grid).len() == 2)
        .collect::<Vec<&GridSymbol>>();

    let sum_of_products = star_symbols_with_two_surrounding_numbers
        .iter()
        .map(|s| s.surrounding_numbers(&grid).iter().product::<usize>())
        .sum::<usize>();

    println!("Part 2: {}", sum_of_products);
}

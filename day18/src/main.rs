use std::collections::HashMap;

static EXAMPLE_INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 18 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let instructions = input
        .lines()
        .map(|line| {
            let (part, _) = line.split_once(" (").unwrap();
            let (direction, distance) = part.split_once(' ').unwrap();

            let distance = distance.parse::<usize>().unwrap();

            (direction, distance)
        })
        .collect::<Vec<_>>();

    let mut grid: HashMap<(isize, isize), char> = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in instructions {
        match direction {
            "U" => {
                for _ in 0..distance {
                    y += 1;
                    grid.insert((x, y), '#');
                }
            }
            "D" => {
                for _ in 0..distance {
                    y -= 1;
                    grid.insert((x, y), '#');
                }
            }
            "L" => {
                for _ in 0..distance {
                    x -= 1;
                    grid.insert((x, y), '#');
                }
            }
            "R" => {
                for _ in 0..distance {
                    x += 1;
                    grid.insert((x, y), '#');
                }
            }
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    let min_x = *grid.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *grid.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    (min_y..=max_y).rev().for_each(|y| {
        (min_x..=max_x).for_each(|x| {
            let c = grid.get(&(x, y)).unwrap_or(&'.');
            print!("{}", c);
        });
        println!();
    });

    println!();

    // turn into char grid
    let mut grid: Vec<Vec<char>> = (min_y..=max_y)
        .rev()
        .map(|y| {
            (min_x..=max_x)
                .map(|x| *grid.get(&(x, y)).unwrap_or(&'.'))
                .collect()
        })
        .collect();

    let inside_point: (usize, usize) = grid
        .iter()
        .skip(1)
        .enumerate()
        .find_map(|(y, row)| {
            let mut inside = false;

            for (x, c) in row.iter().enumerate() {
                if inside && *c == '.' {
                    return Some((x, y + 1));
                }

                inside = *c == '#';
            }

            None
        })
        .unwrap();

    // flood fill
    let mut stack = vec![inside_point];
    while let Some((x, y)) = stack.pop() {
        if grid[y][x] == '.' {
            grid[y][x] = '#';
            stack.push((x + 1, y));
            if x > 0 {
                stack.push((x - 1, y));
            }
            stack.push((x, y + 1));
            if y > 0 {
                stack.push((x, y - 1));
            }
        }
    }

    (0..grid.len()).for_each(|y| {
        (0..grid[y].len()).for_each(|x| {
            print!("{}", grid[y][x]);
        });
        println!();
    });

    let n_filled = grid.iter().flatten().filter(|c| **c == '#').count();

    println!("\nNumber of filled squares: {}", n_filled);
}

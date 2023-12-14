use std::{collections::HashMap, fmt};

static EXAMPLE_INPUT: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let data = input
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = data[0].len();
        let height = data.len();

        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).and_then(|row| row.get(x)).copied()
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
    }

    fn slide(&mut self, direction: &Direction) {
        let mut iter_movement = 0;
        loop {
            for y in 0..self.height {
                for x in 0..self.width {
                    let c = self.get(x, y).unwrap();
                    if c == 'O' {
                        match direction {
                            Direction::North => {
                                // check if we can move north
                                if y == 0 {
                                    // we hit a wall, so we can't move north
                                    continue;
                                }

                                let north = self.get(x, y - 1).unwrap();
                                if north == '#' || north == 'O' {
                                    // we hit a rock or another rock, so we can't move north
                                    continue;
                                }

                                // we can move north, so move the rock
                                self.set(x, y, '.');
                                self.set(x, y - 1, 'O');
                                iter_movement += 1;
                            }
                            Direction::West => {
                                // check if we can move west
                                if x == 0 {
                                    // we hit a wall, so we can't move west
                                    continue;
                                }

                                let west = self.get(x - 1, y).unwrap();
                                if west == '#' || west == 'O' {
                                    // we hit a rock or another rock, so we can't move west
                                    continue;
                                }

                                // we can move west, so move the rock
                                self.set(x, y, '.');
                                self.set(x - 1, y, 'O');
                                iter_movement += 1;
                            }
                            Direction::South => {
                                // check if we can move south
                                if y == self.height - 1 {
                                    // we hit a wall, so we can't move south
                                    continue;
                                }

                                let south = self.get(x, y + 1).unwrap();
                                if south == '#' || south == 'O' {
                                    // we hit a rock or another rock, so we can't move south
                                    continue;
                                }

                                // we can move south, so move the rock
                                self.set(x, y, '.');
                                self.set(x, y + 1, 'O');
                                iter_movement += 1;
                            }
                            Direction::East => {
                                // check if we can move east
                                if x == self.width - 1 {
                                    // we hit a wall, so we can't move east
                                    continue;
                                }

                                let east = self.get(x + 1, y).unwrap();
                                if east == '#' || east == 'O' {
                                    // we hit a rock or another rock, so we can't move east
                                    continue;
                                }

                                // we can move east, so move the rock
                                self.set(x, y, '.');
                                self.set(x + 1, y, 'O');
                                iter_movement += 1;
                            }
                        }
                    }
                }
            }
            if iter_movement == 0 {
                break;
            } else {
                iter_movement = 0;
            }
        }
    }

    fn north_load(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .filter(|&c| c == &'O')
                    .map(|_| self.height - y)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 14 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let start = std::time::Instant::now();
    part1(input);
    println!("Time: {:?}\n", start.elapsed());
    let start = std::time::Instant::now();
    part2(input);
    println!("Time: {:?}", start.elapsed());
}

fn part1(input: &str) {
    let mut grid = Grid::new(input);
    // println!("{}", grid);

    // tilt all rocks north, until they hit a wall, another rock or a '#'
    grid.slide(&Direction::North);
    println!("Part 1: {}", grid.north_load());
}

fn part2(input: &str) {
    let mut grid = Grid::new(input);

    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut curr_iter = 0;
    let mut grid_states: HashMap<Grid, (usize, usize)> = HashMap::new();
    let goal = 1_000_000_000 - 1;
    loop {
        for dir in directions.iter() {
            grid.slide(dir);
        }

        if grid_states.contains_key(&grid) {
            let (iter_first_seen, _) = grid_states.get(&grid).unwrap();
            let cycle_length = curr_iter - iter_first_seen;

            for (iter, load) in grid_states
                .values()
                // has to be inside the cycle
                .filter(|(iter, _)| *iter >= *iter_first_seen)
            {
                if iter % cycle_length == goal % cycle_length {
                    println!(
                        "Part 2: iter {}, cycle length {}, load {}",
                        iter, cycle_length, load
                    );
                    return;
                }
            }
        } else {
            grid_states.insert(grid.clone(), (curr_iter, grid.north_load()));
            curr_iter += 1;
        }
    }
}

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

fn main() {
    println!("\n-- Advent of Code 2023 - Day 14 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    // part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut grid = Grid::new(input);
    println!("{}", grid);

    // tilt all rocks north, until they hit a wall, another rock or a '#'
    let mut iter_movement = 0;
    loop {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let c = grid.get(x, y).unwrap();
                if c == 'O' {
                    // check if we can move north
                    if y == 0 {
                        // we hit a wall, so we can't move north
                        continue;
                    }

                    let north = grid.get(x, y - 1).unwrap();
                    if north == '#' || north == 'O' {
                        // we hit a rock or another rock, so we can't move north
                        continue;
                    }

                    // we can move north, so move the rock
                    grid.set(x, y, '.');
                    grid.set(x, y - 1, 'O');
                    iter_movement += 1;
                }
            }
        }

        if iter_movement == 0 {
            break;
        } else {
            iter_movement = 0;
        }
    }

    let total_load = grid
        .data
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'O')
                .map(|(_, _)| grid.height - y)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Total load: {}", total_load);
}

enum Direction {
    North,
    West,
    South,
    East,
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
                            _ => unreachable!(),
                        }

                        if iter_movement == 0 {
                            break;
                        } else {
                            iter_movement = 0;
                        }
                    }
                }
            }
        }
    }

    fn north_load(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == 'O')
                    .map(|(_, _)| self.height - y)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

fn part2(input: &str) {
    let mut grid = Grid::new(input);
    // println!("{}", grid);

    // Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.
    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut n_iters = 0;
    let mut grid_states: HashMap<Grid, (usize, usize)> = HashMap::new();
    let goal = 1_000_000_000;
    loop {
        for dir in directions.iter() {
            grid.slide(dir);
        }

        if grid_states.contains_key(&grid) {
            let (prev_iter, prev_load) = grid_states.get(&grid).unwrap();
            let cycle = n_iters - prev_iter;
            let cycle_load = grid.north_load();
            let cycle_load_diff = cycle_load - prev_load;
            let remaining_iters = goal - n_iters;
            let remaining_cycles = remaining_iters / cycle;
            let remaining_load = remaining_cycles * cycle_load_diff;
            let total_load = prev_load + remaining_load;
            println!("Total load: {}", total_load);
            break;
        } else {
            grid_states.insert(grid.clone(), (n_iters, grid.north_load()));
            n_iters += 1;
        }
    }
}

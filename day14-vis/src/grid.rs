use std::fmt;

pub enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    data: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
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
    pub fn new(input: &str) -> Self {
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

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y).and_then(|row| row.get(x)).copied()
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
    }

    pub fn slide(&mut self, direction: &Direction) -> bool {
        let mut iter_movement = 0;
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

        iter_movement == 0
    }

    pub fn north_load(&self) -> usize {
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

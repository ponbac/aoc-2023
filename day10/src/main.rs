use core::fmt;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

static EXAMPLE_INPUT: &str = r#"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Map {
    start: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut tiles = Vec::new();

        for line in input.trim().lines() {
            let mut row = Vec::new();

            for c in line.chars() {
                row.push(Tile::from(c));
                if c == 'S' {
                    start = (row.len() - 1, tiles.len());
                }
            }

            tiles.push(row);
        }

        Self { start, tiles }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn expand(&mut self) {
        self.tiles = self
            .tiles
            .iter()
            .flat_map(|row| {
                [
                    row.iter()
                        .flat_map(|tile| tile.expand()[0])
                        .collect::<Vec<Tile>>(),
                    row.iter()
                        .flat_map(|tile| tile.expand()[1])
                        .collect::<Vec<Tile>>(),
                ]
            })
            .collect();
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Tile {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    outside: bool,
}

impl Tile {
    fn expand(&self) -> [[Tile; 2]; 2] {
        [
            [
                *self,
                Tile {
                    west: self.east,
                    east: self.east,
                    ..Default::default()
                },
            ],
            [
                Tile {
                    north: self.south,
                    south: self.south,
                    ..Default::default()
                },
                Tile::from('.'),
            ],
        ]
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            'F' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            '.' => Self {
                ..Default::default()
            },
            'S' => Self {
                ..Default::default()
            },
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 10 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let mut map = Map::new(input);

    let start_pos = map.start;
    println!("{:?}", start_pos);
    // let start = Tile::from('F'); // hardcoded for example input
    let start = Tile::from('|'); // hardcoded for real input
    let start_tile = map.get_mut(start_pos.0, start_pos.1).unwrap();
    *start_tile = start;
    // println!("{}", map);

    let mut distance = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((0, start_pos));
    while let Some((d, (x, y))) = priority_queue.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        distance = distance.max(d);

        // check neighbors
        let tile = map.get(x, y).unwrap();
        if tile.north {
            priority_queue.push(((d + 1), (x, y - 1)));
        }
        if tile.east {
            priority_queue.push(((d + 1), (x + 1, y)));
        }
        if tile.south {
            priority_queue.push(((d + 1), (x, y + 1)));
        }
        if tile.west {
            priority_queue.push(((d + 1), (x - 1, y)));
        }
    }

    println!(
        "Part 1: {}",
        distance / 2 + if distance % 2 == 0 { 0 } else { 1 }
    );

    // clear everything except the loop
    for i in 0..map.tiles.len() {
        for j in 0..map.tiles[i].len() {
            if !visited.contains(&(j, i)) {
                *map.get_mut(j, i).unwrap() = Tile::from('.');
            }
        }
    }
    // println!("{}", map);

    // expand tiles
    let mut expanded_map = map.clone();
    expanded_map.expand();
    // println!("{}", expanded_map);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0)); // might not work for all inputs

    let mut n_outside = 0;
    while let Some((x, y)) = queue.pop_front() {
        if expanded_map.get(x, y).unwrap() != &Tile::from('.') {
            continue;
        }

        let tile = expanded_map.get_mut(x, y).unwrap();
        tile.outside = true;
        if x % 2 == 0 && y % 2 == 0 {
            n_outside += 1;
        }

        // check neighbors
        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if x < expanded_map.tiles[y].len() - 1 {
            queue.push_back((x + 1, y));
        }
        if y < expanded_map.tiles.len() - 1 {
            queue.push_back((x, y + 1));
        }
        if x > 0 {
            queue.push_back((x - 1, y));
        }
    }
    // println!("{}", expanded_map);

    let n_inside = map.tiles.len() * map.tiles[0].len() - n_outside - visited.len();
    println!("Part 2: {}", n_inside);
}

fn _part2(_input: &str) {
    todo!()
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.north, self.east, self.south, self.west, self.outside) {
            (_, _, _, _, true) => write!(f, "O"),
            (false, true, false, true, false) => write!(f, "-"),
            (true, false, true, false, false) => write!(f, "|"),
            (true, true, false, false, false) => write!(f, "L"),
            (true, false, false, true, false) => write!(f, "J"),
            (false, true, true, false, false) => write!(f, "F"),
            (false, false, true, true, false) => write!(f, "7"),
            (false, false, false, false, false) => write!(f, "."),
            _ => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        //         let input = r#"
        // F--7
        // |..|
        // L--J
        //         "#;

        let input = r#"
F-7
|.|
|.|
L-J
        "#;
        let mut map = Map::new(input);
        map.expand();
        println!("{:?}", map);
        println!("{}", map);
    }
}

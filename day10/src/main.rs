use core::fmt;

static EXAMPLE_INPUT: &str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

struct Map {
    start: (i32, i32),
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
                    start = (row.len() as i32 - 1, tiles.len() as i32);
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

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{}", tile);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Horizontal,
    Vertical,
    DownRight,
    DownLeft,
    LeftDown,
    RightDown,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            'L' => Self::DownRight,
            'J' => Self::DownLeft,
            'F' => Self::LeftDown,
            '7' => Self::RightDown,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Self::Horizontal => '-',
            Self::Vertical => '|',
            Self::DownRight => 'L',
            Self::DownLeft => 'J',
            Self::LeftDown => 'F',
            Self::RightDown => '7',
            Self::Ground => '.',
            Self::Start => 'S',
        };
        write!(f, "{}", symbol)
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 10 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let map = Map::new(input);

    map.print()
}

fn _part2(_input: &str) {
    todo!()
}

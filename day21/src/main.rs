use std::{collections::HashSet, iter};

static EXAMPLE_INPUT: &str = r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn add(&self, pos: (isize, isize)) -> (isize, isize) {
        let (x, y) = pos;
        let (dx, dy) = self.delta();
        (x + dx, y + dy)
    }

    fn checked_add(
        &self,
        pos: (isize, isize),
        upper_limit: (isize, isize),
    ) -> Option<(isize, isize)> {
        let (x, y) = pos;
        let (dx, dy) = self.delta();
        let x = x + dx;
        let y = y + dy;

        if x < 0 || y < 0 || x >= upper_limit.0 || y >= upper_limit.1 {
            None
        } else {
            Some((x, y))
        }
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    start_pos: (isize, isize),
    height: isize,
    width: isize,
}

impl Grid {
    fn new(i: &str) -> Self {
        let data = i
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let start_pos = data
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
            })
            .unwrap();

        let height = data.len();
        let width = data[0].len();

        Self {
            data,
            start_pos: (start_pos.0 as isize, start_pos.1 as isize),
            height: height as isize,
            width: width as isize,
        }
    }

    fn get_wrapped(&self, pos: (isize, isize)) -> char {
        let (x, y) = pos;
        let x = x.rem_euclid(self.width);
        let y = y.rem_euclid(self.height);
        self.data[y as usize][x as usize]
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 21 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let grid = Grid::new(input);

    let mut initial_visited = HashSet::new();
    initial_visited.insert(grid.start_pos);

    let reachable = iter::successors(Some(initial_visited), |prev_visited| {
        let mut next = HashSet::new();

        for pos in prev_visited {
            for dir in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(next_pos) = dir.checked_add(*pos, (grid.width, grid.height)) {
                    if grid.get_wrapped(next_pos) == '.' || grid.get_wrapped(next_pos) == 'S' {
                        next.insert(next_pos);
                    }
                }
            }
        }

        Some(next)
    })
    .nth(64)
    .unwrap();

    println!("Part 1: {}", reachable.len());

    let mut initial_visited: HashSet<(isize, isize)> = HashSet::new();
    initial_visited.insert(grid.start_pos);

    let _ = iter::successors(Some(initial_visited), |prev_visited| {
        let mut next = HashSet::new();

        for pos in prev_visited {
            for dir in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next_pos = dir.add(*pos);
                let next_tile = grid.get_wrapped(next_pos);
                match next_tile {
                    '.' | 'S' => {
                        next.insert(next_pos);
                    }
                    _ => {}
                }
            }
        }

        Some(next)
    })
    .enumerate()
    // .inspect(|(i, visited)| match i {
    //     6 | 10 | 50 | 100 | 500 | 1000 | 5000 => {
    //         println!("{}: {}", i, visited.len());
    //     }
    //     _ => {}
    // })
    // .nth(5_000)
    .inspect(|(i, visited)| {
        if i >= &64 && ((i + 1 - 65) % 131) == 0 {
            println!("{:03}: {}", i + 1, visited.len());
        }
    })
    .nth(26_501_365)
    .unwrap();
}

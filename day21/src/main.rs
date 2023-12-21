use std::collections::{HashSet, VecDeque};

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

    fn checked_add(
        &self,
        pos: (usize, usize),
        upper_limit: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (x, y) = pos;
        let (dx, dy) = self.delta();
        let x = x as isize + dx;
        let y = y as isize + dy;

        if x < 0 || y < 0 || x >= upper_limit.0 as isize || y >= upper_limit.1 as isize {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    start_pos: (usize, usize),
    height: usize,
    width: usize,
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
            start_pos,
            height,
            width,
        }
    }

    fn get(&self, pos: (usize, usize)) -> char {
        let (x, y) = pos;
        self.data[y][x]
    }

    fn print(&self) {
        for row in &self.data {
            println!("{}", row.iter().collect::<String>());
        }
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

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut at_step_count = HashSet::new();

    queue.push_back((grid.start_pos, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if steps == 64 {
            at_step_count.insert(pos);
            continue;
        }

        if visited.contains(&(pos, steps)) {
            continue;
        }

        visited.insert((pos, steps));

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_pos) = dir.checked_add(pos, (grid.width, grid.height)) {
                if grid.get(next_pos) == '.' || grid.get(next_pos) == 'S' {
                    queue.push_back((next_pos, steps + 1));
                }
            }
        }
    }

    println!("Part 1: {}", at_step_count.len());

    // Part 2
    let step_goal = 26_501_365;
}

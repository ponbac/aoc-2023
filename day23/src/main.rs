use std::collections::{HashMap, HashSet, VecDeque};

static EXAMPLE_INPUT: &str = r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
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

    fn add(&self, pos: (usize, usize)) -> (usize, usize) {
        let (x, y) = pos;
        let (dx, dy) = self.delta();
        (x + dx as usize, y + dy as usize)
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    data: Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
    height: usize,
    width: usize,
}

impl Grid {
    fn parse(i: &str) -> Self {
        let data = i
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = data.len();
        let width = data[0].len();

        let start = (
            data.first()
                .unwrap()
                .iter()
                .position(|&c| c == '.')
                .unwrap(),
            0,
        );
        let goal = (
            data.iter()
                .last()
                .unwrap()
                .iter()
                .position(|&c| c == '.')
                .unwrap(),
            height - 1,
        );

        Self {
            data,
            start,
            goal,
            height,
            width,
        }
    }

    fn get(&self, pos: (usize, usize)) -> char {
        self.data[pos.1][pos.0]
    }

    fn cleaned(&self) -> Self {
        let mut grid = self.clone();
        for y in 0..grid.height {
            for x in 0..grid.width {
                match grid.get((x, y)) {
                    '^' | 'v' | '<' | '>' => grid.data[y][x] = '.',
                    _ => {}
                }
            }
        }
        grid
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 23 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let grid = Grid::parse(input);

    let part1 = find_longest_path(&grid);
    println!("Part 1: {}", part1);

    let part2 = find_longest_path_2(&grid.cleaned());
    println!("Part 2: {}", part2);
}

fn find_longest_path(grid: &Grid) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back((grid.start, 0, HashSet::from([grid.start])));

    let mut max_steps = 0;
    while let Some((pos, steps, mut visited)) = todo.pop_front() {
        if pos == grid.goal {
            max_steps = max_steps.max(steps);
            continue;
        }

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_pos) = dir.checked_add(pos, (grid.width, grid.height)) {
                if visited.contains(&next_pos) {
                    continue;
                }

                match grid.get(next_pos) {
                    '#' => continue,
                    '.' => {
                        visited.insert(next_pos);
                        todo.push_back((next_pos, steps + 1, visited.clone()));
                    }
                    '^' | 'v' | '<' | '>' => {
                        visited.insert(next_pos);

                        let dir = match grid.get(next_pos) {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => panic!("Unexpected char"),
                        };
                        let next_pos = dir.add(next_pos);
                        if !visited.contains(&next_pos) {
                            visited.insert(next_pos);
                            todo.push_back((next_pos, steps + 2, visited.clone()));
                        }
                    }
                    _ => panic!("Unexpected char"),
                }
            }
        }
    }

    max_steps
}

fn find_longest_path_2(grid: &Grid) -> usize {
    let mut nodes = HashSet::new();

    nodes.insert(grid.start);
    nodes.insert(grid.goal);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get((x, y)) == '.' {
                let mut neighbours = 0;
                for dir in &[
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if let Some(next_pos) = dir.checked_add((x, y), (grid.width, grid.height)) {
                        if grid.get(next_pos) == '.' {
                            neighbours += 1;
                        }
                    }
                }

                if neighbours > 2 {
                    nodes.insert((x, y));
                }
            }
        }
    }

    let mut edges = HashMap::new();
    for node in &nodes {
        edges.insert(*node, find_edges(*node, grid, &nodes));
    }

    let mut todo = VecDeque::new();
    todo.push_back((grid.start, 0, HashSet::from([grid.start])));

    let mut max_steps = 0;
    while let Some((pos, steps, visited)) = todo.pop_front() {
        if pos == grid.goal {
            max_steps = max_steps.max(steps);
            continue;
        }

        for (next_pos, next_steps) in edges.get(&pos).unwrap() {
            if visited.contains(next_pos) {
                continue;
            }

            let mut visited = visited.clone();
            visited.insert(*next_pos);
            todo.push_back((*next_pos, steps + next_steps, visited));
        }
    }

    max_steps
}

fn find_edges(
    start_node: (usize, usize),
    grid: &Grid,
    nodes: &HashSet<(usize, usize)>,
) -> Vec<((usize, usize), usize)> {
    let mut todo = VecDeque::new();
    let mut visited = HashSet::new();
    todo.push_back((start_node, 0));
    visited.insert(start_node);

    let mut result = Vec::new();
    while let Some((pos, steps)) = todo.pop_front() {
        if pos != start_node && nodes.contains(&pos) {
            result.push((pos, steps));
            continue;
        }

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_pos) = dir.checked_add(pos, (grid.width, grid.height)) {
                if grid.get(next_pos) == '.' && visited.insert(next_pos) {
                    todo.push_back((next_pos, steps + 1));
                }
            }
        }
    }

    result
}

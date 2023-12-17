use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

static EXAMPLE_INPUT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
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
    data: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(i: &str) -> Self {
        let data = i
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }

    fn get(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;
        self.data[y][x]
    }

    fn get_p1_neighbors(
        &self,
        position: (usize, usize),
        direction: &Direction,
        steps: usize,
    ) -> Vec<((usize, usize), Direction, usize)> {
        let mut neighbors = Vec::new();

        if steps < 3 {
            if let Some(new_pos) = direction.checked_add(position, (self.width, self.height)) {
                neighbors.push((new_pos, *direction, steps + 1));
            }
        }

        let (left_direction, right_direction) = match direction {
            Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
            Direction::Left | Direction::Right => (Direction::Down, Direction::Up),
        };

        for dir in [left_direction, right_direction].iter() {
            if let Some(new_pos) = dir.checked_add(position, (self.width, self.height)) {
                neighbors.push((new_pos, *dir, 1));
            }
        }

        neighbors
    }

    fn get_p2_neighbors(
        &self,
        position: (usize, usize),
        direction: &Direction,
        steps: usize,
    ) -> Vec<((usize, usize), Direction, usize)> {
        let mut neighbors = Vec::new();

        if steps < 10 {
            if let Some(new_pos) = direction.checked_add(position, (self.width, self.height)) {
                neighbors.push((new_pos, *direction, steps + 1));
            }

            if steps < 4 {
                return neighbors;
            }
        }

        let (left_direction, right_direction) = match direction {
            Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
            Direction::Left | Direction::Right => (Direction::Down, Direction::Up),
        };

        for dir in [left_direction, right_direction].iter() {
            if let Some(new_pos) = dir.checked_add(position, (self.width, self.height)) {
                neighbors.push((new_pos, *dir, 1));
            }
        }

        neighbors
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    heat_loss: usize,
    position: (usize, usize),
    direction: Direction,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_least_heat_loss_path(grid: &Grid, part_2: bool) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let end_pos = (grid.width - 1, grid.height - 1);

    heap.push(State {
        heat_loss: 0,
        position: (0, 0),
        direction: Direction::Right,
        steps: 0,
    });

    while let Some(state) = heap.pop() {
        if state.position == end_pos {
            return state.heat_loss;
        }

        if !visited.insert((state.position, state.direction, state.steps)) {
            continue;
        }

        let neighbors = if part_2 {
            grid.get_p2_neighbors(state.position, &state.direction, state.steps)
        } else {
            grid.get_p1_neighbors(state.position, &state.direction, state.steps)
        };

        for (new_pos, new_dir, new_steps) in neighbors {
            let new_heat_loss = state.heat_loss + grid.get(new_pos);

            heap.push(State {
                heat_loss: new_heat_loss,
                position: new_pos,
                direction: new_dir,
                steps: new_steps,
            });
        }
    }

    unreachable!("there should always be a path to the end!")
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 17 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let start = std::time::Instant::now();
    let grid = Grid::new(input);

    let heat_loss = find_least_heat_loss_path(&grid, false);
    println!("Part 1: {}", heat_loss);

    let heat_loss = find_least_heat_loss_path(&grid, true);
    println!("Part 2: {}", heat_loss);
    println!("Time: {:?}", start.elapsed());
}

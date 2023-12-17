use std::collections::{BinaryHeap, VecDeque};

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

    fn add(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        let (dx, dy) = self.delta();
        let x = x as isize + dx;
        let y = y as isize + dy;
        if x < 0 || y < 0 {
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

    fn get_valid_neighbors(
        &self,
        position: (usize, usize),
        direction: &Direction,
        steps: usize,
    ) -> Vec<((usize, usize), Direction, usize)> {
        let mut neighbors = Vec::new();

        // Continue in the same direction if steps are less than 3
        if steps < 3 {
            if let Some(new_pos) = direction.add(position) {
                if new_pos.0 < self.width && new_pos.1 < self.height {
                    neighbors.push((new_pos, *direction, steps + 1));
                }
            }
        }

        // Turn left and right
        let left_direction = match direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };

        let right_direction = match direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };

        for dir in [left_direction, right_direction].iter() {
            if let Some(new_pos) = dir.add(position) {
                if new_pos.0 < self.width && new_pos.1 < self.height {
                    neighbors.push((new_pos, *dir, 1));
                }
            }
        }

        neighbors
    }

    fn print(&self) {
        for row in &self.data {
            for value in row {
                print!("{}", value);
            }
            println!();
        }
    }

    fn print_with_path(&self, path: &[(usize, usize)]) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if path.contains(&(x, y)) {
                    print!("✖️ ");
                } else {
                    print!("{} ", value);
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    heat_loss: usize,
    position: (usize, usize),
    direction: Direction,
    steps: usize,
    distance_to_end: usize,
    path: Vec<(usize, usize)>,
}

// Implement Ord and PartialOrd to use State in a BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.distance_to_end.cmp(&other.distance_to_end))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_least_heat_loss_path(grid: &Grid) -> usize {
    let mut pq = BinaryHeap::new();
    let mut visited = std::collections::HashSet::new();

    let start_state = State {
        heat_loss: 0,
        position: (0, 0),
        direction: Direction::Right,
        steps: 0,
        distance_to_end: grid.width + grid.height - 2,
        path: Vec::new(),
    };

    pq.push(start_state);

    while let Some(State {
        heat_loss,
        position,
        direction,
        steps,
        distance_to_end: _,
        path,
    }) = pq.pop()
    {
        if position == (grid.width - 1, grid.height - 1) {
            grid.print_with_path(&path);
            return heat_loss;
        }

        let visit_key = (position.0, position.1, direction);
        if visited.contains(&visit_key) {
            continue;
        }
        visited.insert(visit_key);

        for (next_position, next_direction, next_steps) in
            grid.get_valid_neighbors(position, &direction, steps)
        {
            let next_heat_loss = heat_loss + grid.get(next_position);
            let next_distance_to_end = grid.width + grid.height - next_position.0 - next_position.1;
            let mut next_path = path.clone();
            next_path.push(next_position);
            let next_state = State {
                heat_loss: next_heat_loss,
                position: next_position,
                direction: next_direction,
                steps: next_steps,
                distance_to_end: next_distance_to_end,
                path: next_path,
            };
            pq.push(next_state);
        }
    }

    usize::MAX // Return maximum value if no path found
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 17 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    part1(input.trim());
    // part2(input.trim());
}

fn part1(input: &str) {
    let grid = Grid::new(input);
    // grid.print();

    let heat_loss = find_least_heat_loss_path(&grid);
    println!("Part 1: {}", heat_loss);
}

fn part2(input: &str) {
    todo!()
}

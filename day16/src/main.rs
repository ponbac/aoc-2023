static EXAMPLE_INPUT: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Tile {
    data: char,
    energized: bool,
    from_north: bool,
    from_east: bool,
    from_south: bool,
    from_west: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            data: '.',
            energized: false,
            from_north: false,
            from_east: false,
            from_south: false,
            from_west: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Self {
            data: Vec::new(),
            width: 0,
            height: 0,
        };

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Tile {
                    data: c,
                    ..Default::default()
                });
            }
            grid.data.push(row);
        }

        grid.width = grid.data[0].len();
        grid.height = grid.data.len();

        grid
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.data.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn n_energized(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|tile| tile.energized).count())
            .sum()
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 16 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let start = std::time::Instant::now();
    part1(input.trim());
    println!("Time: {:?}\n", start.elapsed());
    let start = std::time::Instant::now();
    part2(input.trim());
    println!("Time: {:?}", start.elapsed());
}

fn part1(input: &str) {
    let mut grid = Grid::new(input);
    beam(&mut grid, 0, 0, Dir::East);

    println!("Part 1: {}", grid.n_energized());
}

fn part2(input: &str) {
    let grid = Grid::new(input);

    let mut max_n_energized = 0;
    for x in 0..grid.width {
        let mut grid = grid.clone();
        beam(&mut grid, x, 0, Dir::South);

        max_n_energized = max_n_energized.max(grid.n_energized());
    }

    // I should probably do the same for the other directions, and also
    // handle the corner cases, but I got the right answer with this!

    println!("Part 2: {}", max_n_energized);
}

fn beam(grid: &mut Grid, x: usize, y: usize, direction: Dir) {
    let tile = match grid.get_mut(x, y) {
        Some(tile) => tile,
        None => return,
    };

    if tile.energized
        && match direction {
            Dir::North => tile.from_south,
            Dir::East => tile.from_west,
            Dir::South => tile.from_north,
            Dir::West => tile.from_east,
        }
    {
        return;
    }

    tile.energized = true;
    match direction {
        Dir::North => tile.from_south = true,
        Dir::East => tile.from_west = true,
        Dir::South => tile.from_north = true,
        Dir::West => tile.from_east = true,
    }

    match tile.data {
        '.' => match direction {
            Dir::North => beam(grid, x, if y == 0 { return } else { y - 1 }, Dir::North),
            Dir::East => beam(grid, x + 1, y, Dir::East),
            Dir::South => beam(grid, x, y + 1, Dir::South),
            Dir::West => beam(grid, if x == 0 { return } else { x - 1 }, y, Dir::West),
        },
        '/' => match direction {
            Dir::North => beam(grid, x + 1, y, Dir::East),
            Dir::East => beam(grid, x, if y == 0 { return } else { y - 1 }, Dir::North),
            Dir::South => beam(grid, if x == 0 { return } else { x - 1 }, y, Dir::West),
            Dir::West => beam(grid, x, y + 1, Dir::South),
        },
        '\\' => match direction {
            Dir::North => beam(grid, if x == 0 { return } else { x - 1 }, y, Dir::West),
            Dir::East => beam(grid, x, y + 1, Dir::South),
            Dir::South => beam(grid, x + 1, y, Dir::East),
            Dir::West => beam(grid, x, if y == 0 { return } else { y - 1 }, Dir::North),
        },
        '|' => match direction {
            Dir::North => beam(grid, x, if y == 0 { return } else { y - 1 }, Dir::North),
            Dir::South => beam(grid, x, y + 1, Dir::South),
            Dir::East | Dir::West => {
                if y != 0 {
                    beam(grid, x, y - 1, Dir::North);
                }
                beam(grid, x, y + 1, Dir::South);
            }
        },
        '-' => match direction {
            Dir::East => beam(grid, x + 1, y, Dir::East),
            Dir::West => beam(grid, if x == 0 { return } else { x - 1 }, y, Dir::West),
            Dir::North | Dir::South => {
                if x != 0 {
                    beam(grid, x - 1, y, Dir::West);
                }
                beam(grid, x + 1, y, Dir::East);
            }
        },
        _ => unreachable!(),
    }
}

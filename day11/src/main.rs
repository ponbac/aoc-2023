use core::fmt;

static EXAMPLE_INPUT: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    data: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let empty_rows: Vec<usize> = data
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|&c| c == '.'))
            .map(|(i, _)| i)
            .collect();

        let empty_cols: Vec<usize> = (0..data[0].len())
            .filter(|&col| data.iter().all(|row| row[col] == '.'))
            .collect();

        Self {
            data,
            empty_rows,
            empty_cols,
        }
    }

    // e.g. walk from (3, 0) to (7, 1) should be 6 steps
    fn walk_distance_between(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        n_expand: usize,
    ) -> usize {
        let mut distance = 0;
        let mut current = start;

        // if an empty row or col is passed, count it n_expand times
        loop {
            let (x, y) = current;
            let next = if x < end.0 {
                (x + 1, y)
            } else if x > end.0 {
                (x - 1, y)
            } else if y < end.1 {
                (x, y + 1)
            } else if y > end.1 {
                (x, y - 1)
            } else {
                break;
            };

            if self.empty_rows.contains(&y) || self.empty_cols.contains(&x) {
                distance += n_expand;
            } else {
                distance += 1;
            }

            current = next;
        }

        distance
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 11 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let grid = Grid::new(input);

    // println!("Grid:\n{}", grid);
    println!("Empty cols: {:?}", grid.empty_cols);
    println!("Empty rows: {:?}", grid.empty_rows);

    let galaxies: Vec<(usize, usize)> = grid
        .data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    // println!("Galaxies: {:?}", galaxies);

    // calc distance between each pair of galaxies
    let mut part1_distances = Vec::new();
    let mut part2_distances = Vec::new();
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in &galaxies[i + 1..] {
            let p1_distance = grid.walk_distance_between(*galaxy, *other_galaxy, 2);
            let p2_distance = grid.walk_distance_between(*galaxy, *other_galaxy, 1_000_000);

            part1_distances.push(p1_distance);
            part2_distances.push(p2_distance);
        }
    }

    println!("Part 1: {:?}", part1_distances.iter().sum::<usize>());
    println!("Part 2: {:?}", part2_distances.iter().sum::<usize>());
}

fn _part2(_input: &str) {
    todo!()
}

use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

static EXAMPLE_INPUT: &str = r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    // id: char,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn parse(i: &str) -> Self {
        let (start, end) = i.split_once('~').unwrap();

        let start = start
            .split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let end = end
            .split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            start: (start[0], start[1], start[2]),
            end: (end[0], end[1], end[2]),
        }
    }

    fn occupies(&self) -> Vec<(usize, usize, usize)> {
        let mut v = Vec::new();

        let (x1, y1, z1) = self.start;
        let (x2, y2, z2) = self.end;

        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    v.push((x, y, z));
                }
            }
        }

        v
    }

    fn down(&self) -> Self {
        let new_brick = Self {
            start: (self.start.0, self.start.1, self.start.2 - 1),
            end: (self.end.0, self.end.1, self.end.2 - 1),
        };

        if new_brick.start.2 == 0 {
            *self
        } else {
            new_brick
        }
    }
}

fn main() {
    println!("\n-- Advent of Code 2023 - Day 22 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    let start = std::time::Instant::now();
    solve(input.trim());
    println!("Time: {:?}", start.elapsed());
}

fn solve(input: &str) {
    let mut bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();
    bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));

    let mut occupied = HashSet::new();
    for brick in bricks.iter() {
        occupied.extend(brick.occupies());
    }

    fall(&mut bricks, &mut occupied);
    // for brick in bricks.iter() {
    //     println!("{:?}", brick);
    // }

    let removable_bricks = count_removable_bricks(&bricks);
    println!("Part 1: {}", removable_bricks.len());

    let non_removable_bricks = bricks
        .iter()
        .enumerate()
        .filter(|(i, _)| !removable_bricks.contains(i))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let would_fall_sum = non_removable_bricks
        .par_iter()
        .map(|&i| {
            let mut bricks = bricks.clone();
            bricks.remove(i);
            let mut occupied = HashSet::new();
            for brick in bricks.iter() {
                occupied.extend(brick.occupies());
            }
            fall(&mut bricks, &mut occupied)
        })
        .sum::<usize>();
    println!("Part 2: {}", would_fall_sum);
}

fn count_removable_bricks(bricks: &[Brick]) -> Vec<usize> {
    let mut removable_bricks = Vec::new();

    let mut occupied = HashSet::new();
    for brick in bricks {
        occupied.extend(brick.occupies());
    }

    for i in 0..bricks.len() {
        // temporarily remove the current brick
        // println!("removing brick {}", i);
        for &cell in &bricks[i].occupies() {
            occupied.remove(&cell);
        }

        let is_removable = bricks.iter().enumerate().all(|(j, brick)| {
            let owned_cells = brick.occupies().into_iter().collect::<HashSet<_>>();
            if i == j {
                true // skip the current brick
            } else {
                // check if any cell of the brick either is on the lowest level or has support below by another brick
                brick.occupies().iter().any(|&(x, y, z)| {
                    z == 1
                        || (occupied.contains(&(x, y, z - 1))
                            && !owned_cells.contains(&(x, y, z - 1)))
                })
            }
        });

        if is_removable {
            removable_bricks.push(i);
        }
        // restore the removed brick
        for &cell in &bricks[i].occupies() {
            occupied.insert(cell);
        }
    }

    removable_bricks
}

fn fall(bricks: &mut [Brick], occupied: &mut HashSet<(usize, usize, usize)>) -> usize {
    let mut n_fallen = 0;
    for brick in bricks.iter_mut() {
        let mut has_fallen = false;
        loop {
            if brick.start.2 == 1 {
                if has_fallen {
                    n_fallen += 1;
                }
                break;
            }

            // check if the positions below the brick are free
            let curr_occupies = brick.occupies().into_iter().collect::<HashSet<_>>();
            if brick
                .down()
                .occupies()
                .iter()
                .any(|cell| occupied.contains(cell) && !curr_occupies.contains(cell))
            {
                if has_fallen {
                    n_fallen += 1;
                }
                break; // break if there's a collision
            }

            for cell in brick.occupies() {
                occupied.remove(&cell);
            }
            for cell in brick.down().occupies() {
                occupied.insert(cell);
            }

            *brick = brick.down();
            has_fallen = true;
        }
    }

    n_fallen
}

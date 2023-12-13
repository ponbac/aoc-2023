static EXAMPLE_INPUT: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 13 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let input = input.trim().split("\n\n").collect::<Vec<_>>();

    let grids = input
        .iter()
        .map(|grid| {
            grid.trim()
                .split('\n')
                .map(|row| row.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut sum_2 = 0;
    for grid in &grids {
        let (lines_above_horizontal_opt, columns_left_vertical_opt) =
            find_reflections(grid, None, None);

        let lines_above_horizontal = lines_above_horizontal_opt.unwrap_or(0);
        let columns_left_vertical = columns_left_vertical_opt.unwrap_or(0);

        sum += lines_above_horizontal * 100 + columns_left_vertical;

        // Part 2
        // for every char in the grid, try flipping its value and see if any reflections are found
        'outer: for i in 0..grid.len() {
            if lines_above_horizontal > 0 && i == lines_above_horizontal - 1 {
                continue;
            }
            for j in 0..grid[0].len() {
                if columns_left_vertical > 0 && j == columns_left_vertical - 1 {
                    continue;
                }
                let mut grid = grid.clone();
                grid[i][j] = match grid[i][j] {
                    '#' => '.',
                    '.' => '#',
                    _ => panic!("Unknown char"),
                };

                let (lines_above_horizontal, columns_left_vertical) =
                    find_reflections(&grid, lines_above_horizontal_opt, columns_left_vertical_opt);

                let lines_above_horizontal = lines_above_horizontal.unwrap_or(0);
                let columns_left_vertical = columns_left_vertical.unwrap_or(0);

                if lines_above_horizontal != 0 || columns_left_vertical != 0 {
                    for row in &grid {
                        println!("{}", row.iter().collect::<String>());
                    }

                    println!(
                        "Part 2: lines_above_horizontal: {}, columns_left_vertical: {}",
                        lines_above_horizontal, columns_left_vertical
                    );
                    sum_2 += lines_above_horizontal * 100 + columns_left_vertical;
                    break 'outer;
                }
            }
        }
    }

    println!("\nPart 1: {}\n", sum);
    println!("\nPart 2: {}\n", sum_2);
}

fn part2(_input: &str) {
    todo!()
}

fn find_reflections(
    grid: &Vec<Vec<char>>,
    ignore_hor: Option<usize>,
    ignore_ver: Option<usize>,
) -> (Option<usize>, Option<usize>) {
    // row for row, find identical rows next to each other
    let mut i = None;
    let identical_horizontal_start_index = 'outer: loop {
        i = match i {
            None => Some(0),
            Some(i) => Some(i + 1),
        };

        let i = i.unwrap();
        if i >= grid.len() - 1 {
            break None;
        }

        let row = &grid[i];
        let next_row = &grid[i + 1];

        if row == next_row {
            if ignore_hor.is_some() && ignore_hor.unwrap() == i + 1 {
                continue;
            }
            // check if the next row + 1 is also identical to the current row - 1, and so on
            let mut ii: i32 = i as i32 - 1;
            let mut j: i32 = ii + 3;
            loop {
                if ii < 0 || j >= grid.len() as i32 {
                    break 'outer Some(i + 1);
                }

                let u_ii = ii as usize;
                let u_j = j as usize;
                let row = &grid[u_ii];
                let next_row = &grid[u_j];

                if row == next_row {
                    ii -= 1;
                    j += 1;
                } else {
                    break;
                }
            }
        }
    };

    let mut i = None;
    let identical_vertical_start_index = 'outer: loop {
        i = match i {
            None => Some(0),
            Some(i) => Some(i + 1),
        };

        let i = i.unwrap();
        if i >= grid[0].len() - 1 {
            break None;
        }

        let mut column = Vec::new();
        let mut next_column = Vec::new();
        for row in grid {
            if i < row.len() - 1 {
                column.push(row[i]);
                next_column.push(row[i + 1]);
            } else {
                // Handle the case when i is at the last index of the row
            }
        }

        if column == next_column {
            if ignore_ver.is_some() && ignore_ver.unwrap() == i + 1 {
                continue;
            }
            // check if the next column + 1 is also identical to the current column - 1, and so on
            let mut ii: i32 = i as i32 - 1;
            let mut j: i32 = ii + 3;
            loop {
                if ii < 0 || j >= grid[0].len() as i32 {
                    break 'outer Some(i + 1);
                }

                let u_ii = ii as usize;
                let u_j = j as usize;
                let mut column = Vec::new();
                let mut next_column = Vec::new();
                for row in grid {
                    if u_ii < row.len() - 1 {
                        column.push(row[u_ii]);
                        next_column.push(row[u_j]);
                    } else {
                        // Handle the case when i is at the last index of the row
                    }
                }

                if column == next_column {
                    ii -= 1;
                    j += 1;
                } else {
                    break;
                }
            }
        }
    };

    (
        identical_horizontal_start_index,
        identical_vertical_start_index,
    )
}

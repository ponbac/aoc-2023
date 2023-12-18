static EXAMPLE_INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 18 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let instructions = input
        .lines()
        .map(|line| {
            let (part, _) = line.split_once(" (").unwrap();
            let (dir, distance) = part.split_once(' ').unwrap();

            let dir = dir.chars().next().unwrap();
            let distance = distance.parse::<usize>().unwrap();

            (dir, distance)
        })
        .collect::<Vec<_>>();

    let area = calc_area(&instructions);
    println!("Part 1: {}", area);

    let instructions = input
        .lines()
        .map(|line| {
            let (_, color) = line.split_once(" (#").unwrap();
            let dir = match color.as_bytes()[color.len() - 2] {
                b'0' => 'R',
                b'1' => 'D',
                b'2' => 'L',
                b'3' => 'U',
                _ => unreachable!(),
            };
            let distance = usize::from_str_radix(&color[0..color.len() - 2], 16).unwrap();

            (dir, distance)
        })
        .collect::<Vec<_>>();

    let area = calc_area(&instructions);
    println!("Part 2: {}", area);
}

fn calc_area(instructions: &[(char, usize)]) -> usize {
    let mut curr_pos: (isize, isize) = (0, 0);
    let mut visited = Vec::new();
    let mut boundary = 0;

    for (direction, distance) in instructions {
        visited.push(curr_pos);
        boundary += *distance;

        match direction {
            'R' => curr_pos.0 += *distance as isize,
            'L' => curr_pos.0 -= *distance as isize,
            'U' => curr_pos.1 += *distance as isize,
            'D' => curr_pos.1 -= *distance as isize,
            _ => unreachable!(),
        }
    }

    // Pick's theorem `Area = inside + boundary / 2 - 1` can be rearranged to `inside = Area - boundary / 2 + 1`
    let area = shoelace(&visited);
    let inside = area - boundary / 2 + 1;

    inside + boundary
}

/// Shoelace formula for calculating the area of a polygon.
///
/// https://www.youtube.com/watch?v=FSWPX0XB7a0
fn shoelace(points: &[(isize, isize)]) -> usize {
    let mut area = 0;
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];

        area += (x1 * y2) - (y1 * x2);
    }

    if area < 0 {
        area *= -1;
    }

    area as usize / 2
}

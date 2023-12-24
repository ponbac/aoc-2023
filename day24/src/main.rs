static EXAMPLE_INPUT: &str = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 24 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    solve(input.trim());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hailstone {
    fn parse(i: &str) -> Self {
        let (coords, velocity) = i.split_once(" @ ").unwrap();

        let coords = coords
            .split(',')
            .map(|c| c.trim().parse().unwrap())
            .collect::<Vec<_>>();
        let velocity = velocity
            .split(',')
            .map(|c| c.trim().parse().unwrap())
            .collect::<Vec<_>>();

        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            vx: velocity[0],
            vy: velocity[1],
            vz: velocity[2],
        }
    }

    /// Ignores z-plane
    fn as_line(&self) -> (i64, i64, i64) {
        let a = self.vy;
        let b = -self.vx;
        let c = self.vx * self.y - self.vy * self.x;

        (a, b, c)
    }

    fn intersection(&self, other: &Self) -> Option<(i64, i64)> {
        let (a1, b1, c1) = self.as_line();
        let (a2, b2, c2) = other.as_line();

        let det = a1 * b2 - a2 * b1;
        if det == 0 {
            return None;
        }

        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;

        Some((-x, -y))
    }
}

fn solve(input: &str) {
    let stones = input.lines().map(Hailstone::parse).collect::<Vec<_>>();

    let pos_min = 7;
    let pos_max = 27;

    let mut n_collisions = 0;
    for i in 0..stones.len() {
        for j in i + 1..stones.len() {
            if i == j {
                continue;
            }

            if let Some((x, y)) = stones[i].intersection(&stones[j]) {
                if x >= pos_min && x <= pos_max && y >= pos_min && y <= pos_max {
                    n_collisions += 1;
                }
            }
        }
    }

    println!("Part 1: {}", n_collisions);
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("19, 13, 30 @ -2,  1, -2", "18, 19, 22 @ -1, -1, -2", Some((14, 15)))]
    #[case("19, 13, 30 @ -2,  1, -2", "20, 25, 34 @ -2, -2, -4", Some((11, 16)))]
    fn test_hailstone(
        #[case] input1: &str,
        #[case] input2: &str,
        #[case] expected: Option<(i64, i64)>,
    ) {
        let h1 = Hailstone::parse(input1);
        let h2 = Hailstone::parse(input2);

        assert_eq!(h1.intersection(&h2), expected);
    }
}

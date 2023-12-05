use std::ops::Range;

static EXAMPLE_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

fn main() {
    println!("-- Advent of Code 2023 - Day 5 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    let now = std::time::Instant::now();
    part2(input);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn part1(input: &str) {
    let mut parts = input.split("\n\n");

    let seeds: Vec<i64> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let sections: Vec<_> = parts.map(parse_sections).collect();
    let min_location = seeds
        .into_iter()
        .map(|seed| {
            sections.iter().fold(seed, |seed, section| {
                section
                    .iter()
                    .find_map(|(range, offset)| range.contains(&seed).then_some(seed + offset))
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap();

    println!("Part 1: {}", min_location);
}

fn part2(input: &str) {
    let mut parts = input.split("\n\n");

    let seeds: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0], s[1]))
        .collect();

    let sections: Vec<_> = parts.map(parse_sections).collect();
    let min_location = seeds
        .into_iter()
        .flat_map(|(start, len)| {
            (start..start + len).map(|seed| {
                sections.iter().fold(seed, |seed, section| {
                    section
                        .iter()
                        .find_map(|(range, offset)| range.contains(&seed).then_some(seed + offset))
                        .unwrap_or(seed)
                })
            })
        })
        .min()
        .unwrap();

    println!("Part 2: {}", min_location);
}

fn parse_sections(input: &str) -> Vec<(Range<i64>, i64)> {
    input
        .lines()
        .skip(1)
        .map(|l| {
            let numbers: Vec<i64> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();

            let range = numbers[1]..numbers[1] + numbers[2];
            let offset = numbers[0] - numbers[1];
            (range, offset)
        })
        .collect()
}

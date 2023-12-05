use std::{collections::HashMap, ops::Range};

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

struct Seed {
    id: u64,
    soil: u64,
    fertilizer: u64,
    water: u64,
    light: u64,
    temperature: u64,
    humidity: u64,
    location: u64,
}

fn parse_seeds(i: &str) -> Vec<Seed> {
    let x = i.lines().next().unwrap();
    let x = x.trim_start_matches("seeds: ");
    let x = x.split_whitespace();
    // numbers are a tuple, start index and length
    // take the numbers two at a time
    let numbers = x.map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let chunks = numbers.chunks_exact(2).collect::<Vec<_>>();
    let seed_id_ranges: Vec<(u64, u64)> = chunks
        .iter()
        .map(|c| (c[0], c[1]))
        .collect::<Vec<(u64, u64)>>();

    println!("{:?}", seed_id_ranges);

    let mut seed_to_soil_map: HashMap<u64, u64> = HashMap::new();
    let mut soil_to_fertilizer_map: HashMap<u64, u64> = HashMap::new();
    let mut fertilizer_to_water_map: HashMap<u64, u64> = HashMap::new();
    let mut water_to_light_map: HashMap<u64, u64> = HashMap::new();
    let mut light_to_temperature_map: HashMap<u64, u64> = HashMap::new();
    let mut temperature_to_humidity_map: HashMap<u64, u64> = HashMap::new();
    let mut humidity_to_location_map: HashMap<u64, u64> = HashMap::new();

    let seed_to_soil_line_idx = i
        .lines()
        .position(|l| l.contains("seed-to-soil map:"))
        .unwrap();
    let seed_to_soil_lines = i
        .lines()
        .skip(seed_to_soil_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting seed to soil map");
    for line in seed_to_soil_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        // only care about src where src is in seed_ids
        println!("filtering");
        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| {
                seed_id_ranges
                    .iter()
                    .any(|(start, len)| src >= start && src < &(start + len))
            })
            .collect::<Vec<_>>();

        println!("looping over src range with len {}", relevant_pairs.len());
        for (dest, src) in relevant_pairs {
            seed_to_soil_map.insert(src, dest);
        }
    }

    let soil_ids = seed_to_soil_map.values().copied().collect::<Vec<_>>();
    let soil_to_fertilizer_line_idx = i
        .lines()
        .position(|l| l.contains("soil-to-fertilizer map:"))
        .unwrap();
    let soil_to_fertilizer_lines = i
        .lines()
        .skip(soil_to_fertilizer_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting soil to fertilizer map");
    for line in soil_to_fertilizer_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| soil_ids.contains(src))
            .collect::<Vec<_>>();

        for (dest, src) in relevant_pairs {
            soil_to_fertilizer_map.insert(src, dest);
        }
    }

    let fertilizer_ids = soil_to_fertilizer_map.values().copied().collect::<Vec<_>>();
    let fertilizer_to_water_line_idx = i
        .lines()
        .position(|l| l.contains("fertilizer-to-water map:"))
        .unwrap();
    let fertilizer_to_water_lines = i
        .lines()
        .skip(fertilizer_to_water_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting fertilizer to water map");
    for line in fertilizer_to_water_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| fertilizer_ids.contains(src))
            .collect::<Vec<_>>();

        for (dest, src) in relevant_pairs {
            fertilizer_to_water_map.insert(src, dest);
        }
    }

    let water_ids = fertilizer_to_water_map
        .values()
        .copied()
        .collect::<Vec<_>>();
    let water_to_light_line_idx = i
        .lines()
        .position(|l| l.contains("water-to-light map:"))
        .unwrap();
    let water_to_light_lines = i
        .lines()
        .skip(water_to_light_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting water to light map");
    for line in water_to_light_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| water_ids.contains(src))
            .collect::<Vec<_>>();

        for (dest, src) in relevant_pairs {
            water_to_light_map.insert(src, dest);
        }
    }

    let light_ids = water_to_light_map.values().copied().collect::<Vec<_>>();
    let light_to_temperature_line_idx = i
        .lines()
        .position(|l| l.contains("light-to-temperature map:"))
        .unwrap();
    let light_to_temperature_lines = i
        .lines()
        .skip(light_to_temperature_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting light to temperature map");
    for line in light_to_temperature_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| light_ids.contains(src))
            .collect::<Vec<_>>();

        for (dest, src) in relevant_pairs {
            light_to_temperature_map.insert(src, dest);
        }
    }

    let temperature_ids = light_to_temperature_map
        .values()
        .copied()
        .collect::<Vec<_>>();
    let temperature_to_humidity_line_idx = i
        .lines()
        .position(|l| l.contains("temperature-to-humidity map:"))
        .unwrap();
    let temperature_to_humidity_lines = i
        .lines()
        .skip(temperature_to_humidity_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting temperature to humidity map");
    for line in temperature_to_humidity_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| temperature_ids.contains(src))
            .collect::<Vec<_>>();

        for (dest, src) in relevant_pairs {
            temperature_to_humidity_map.insert(src, dest);
        }
    }

    let humidity_ids = temperature_to_humidity_map
        .values()
        .copied()
        .collect::<Vec<_>>();
    let humidity_to_location_line_idx = i
        .lines()
        .position(|l| l.contains("humidity-to-location map:"))
        .unwrap();
    let humidity_to_location_lines = i
        .lines()
        .skip(humidity_to_location_line_idx + 1)
        .take_while(|l| !l.is_empty());
    println!("Getting humidity to location map");
    for line in humidity_to_location_lines {
        let mut line = line.split_whitespace();
        let dest_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_len = line.next().unwrap().parse::<u64>().unwrap();

        let dest_range = dest_range_start..(dest_range_start + range_len);
        let src_range = src_range_start..(src_range_start + range_len);

        let relevant_pairs = dest_range
            .zip(src_range)
            .filter(|(_, src)| humidity_ids.contains(src))
            .collect::<Vec<_>>();

        for (dest, src) in relevant_pairs {
            humidity_to_location_map.insert(src, dest);
        }
    }

    let mut seeds = Vec::new();
    // for seed_id in seed_ids {
    //     let soil = *seed_to_soil_map.get(&seed_id).unwrap_or(&seed_id);
    //     let fertilizer = *soil_to_fertilizer_map.get(&soil).unwrap_or(&soil);
    //     let water = *fertilizer_to_water_map
    //         .get(&fertilizer)
    //         .unwrap_or(&fertilizer);
    //     let light = *water_to_light_map.get(&water).unwrap_or(&water);
    //     let temperature = *light_to_temperature_map.get(&light).unwrap_or(&light);
    //     let humidity = *temperature_to_humidity_map
    //         .get(&temperature)
    //         .unwrap_or(&temperature);
    //     let location = *humidity_to_location_map.get(&humidity).unwrap_or(&humidity);

    //     let seed = Seed {
    //         id: seed_id,
    //         soil,
    //         fertilizer,
    //         water,
    //         light,
    //         temperature,
    //         humidity,
    //         location,
    //     };
    //     seeds.push(seed);
    // }

    seeds
}

fn main() {
    println!("-- Advent of Code 2023 - Day 5 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    // part1(input);
    let now = std::time::Instant::now();
    part2(input);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn part1(input: &str) {
    let seeds = parse_seeds(input);

    let lowest_location_seed = seeds.iter().min_by_key(|s| s.location).unwrap();

    println!(
        "The seed with the lowest location is seed {} with location {}",
        lowest_location_seed.id, lowest_location_seed.location
    );
}

fn part2(input: &str) {
    // let mut parts = input.split("\r\n\r\n");
    let mut parts = input.split("\n\n"); // example line endings...

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

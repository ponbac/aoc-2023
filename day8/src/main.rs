use std::collections::HashMap;

static EXAMPLE_INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 8 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    part1(input);
    // part2(input);
}

fn part1(input: &str) {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let steps = input.lines().next().unwrap();
    for line in input.lines().skip(2) {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();

        let mut value_parts = value.split(", ");
        let left = value_parts
            .next()
            .unwrap()
            .trim_matches('(')
            .trim_matches(')');
        let right = value_parts
            .next()
            .unwrap()
            .trim_matches('(')
            .trim_matches(')');

        map.insert(key, (left, right));
    }
    let start = input.lines().nth(2).unwrap().split(" = ").next().unwrap();
    let goal = input.lines().last().unwrap().split(" = ").next().unwrap();

    println!("{:?}, len {}", steps, steps.len());
    println!("{:?}", map);
    println!("{:?}", start);
    println!("{:?}", goal);

    let mut current = start;
    let mut n_steps = 0;
    // repeat steps if it ends before the last step
    // while current != goal {
    //     let step = steps.chars().nth(n_steps % steps.len()).unwrap();
    //     // if one of the steps is the goal, go there
    //     if step == 'L' && map.get(current).unwrap().0 == goal {
    //         current = goal;
    //         n_steps += 1;
    //         println!("Left :{}: {}", n_steps, current);
    //         break;
    //     } else if step == 'R' && map.get(current).unwrap().1 == goal {
    //         current = goal;
    //         n_steps += 1;
    //         println!("{}: {}", n_steps, current);
    //         break;
    //     }

    //     current = match step {
    //         'L' => map.get(current).unwrap().0,
    //         'R' => map.get(current).unwrap().1,
    //         _ => panic!("Unknown step: {}", step),
    //     };
    //     n_steps += 1;
    //     println!("{}: {}", n_steps, current);
    // }
    for curr in steps.chars().cycle() {
        n_steps += 1;
        let location = map.get(current).unwrap();
        match curr {
            'L' => current = location.0,
            'R' => current = location.1,
            _ => panic!("invalid direction {curr}"),
        }

        if current == goal {
            break;
        }
    }

    println!("Steps: {}", n_steps);
}

fn part2(input: &str) {
    todo!()
}

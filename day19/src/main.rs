use std::collections::HashMap;

use parse::{Part, Rule, Workflow};

mod parse;

static EXAMPLE_INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 19 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| Workflow::parse(line).unwrap().1)
        .map(|workflow| (workflow.id.clone(), workflow))
        .collect::<HashMap<_, _>>();
    let parts = parts
        .lines()
        .map(|line| Part::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    let sum = parts
        .iter()
        .filter(|&part| process(part, "in", &workflows))
        .map(|part| part.sum())
        .sum::<usize>();
    println!("Part 1: {}", sum);

    let processable_parts = n_processable([(1, 4000); 4], "in", 0, &workflows);
    println!("Part 2: {}", processable_parts);
}

fn process(part: &Part, workflow: &str, workflows: &HashMap<String, Workflow>) -> bool {
    if workflow == "A" {
        return true;
    } else if workflow == "R" {
        return false;
    }

    let workflow = workflows.get(workflow).unwrap();
    for rule in &workflow.rules {
        match rule {
            Rule::GreaterThan(value_id, cmp_v, target)
            | Rule::LessThan(value_id, cmp_v, target) => {
                match (rule, part.get(value_id)) {
                    (Rule::GreaterThan(_, _, _), part_v) if part_v <= *cmp_v => continue,
                    (Rule::LessThan(_, _, _), part_v) if part_v >= *cmp_v => continue,
                    _ => {}
                }

                return process(part, target, workflows);
            }
            Rule::Accept => return true,
            Rule::Reject => return false,
            Rule::Forward(target) => return process(part, target, workflows),
        }
    }

    unreachable!()
}

fn n_processable(
    ranges: [(usize, usize); 4],
    workflow_key: &str,
    rule_index: usize,
    workflows: &HashMap<String, Workflow>,
) -> usize {
    let range_index = |value_id: &String| match value_id.as_str() {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => unreachable!(),
    };

    let sum_ranges = |ranges: &[(usize, usize)]| {
        ranges
            .iter()
            .map(|(min, max)| max - min + 1)
            .product::<usize>()
    };

    if workflow_key == "A" {
        return sum_ranges(&ranges);
    } else if workflow_key == "R" {
        return 0;
    }

    let workflow = workflows.get(workflow_key).unwrap();
    match workflow.rules.get(rule_index).unwrap() {
        Rule::GreaterThan(value_id, cmp_v, target) => {
            let (min, max) = ranges[range_index(value_id)];

            if max <= *cmp_v {
                n_processable(ranges, workflow_key, rule_index + 1, workflows)
            } else if min > *cmp_v {
                n_processable(ranges, target, 0, workflows)
            } else {
                let fail_ranges = {
                    let mut r = ranges;
                    r[range_index(value_id)] = (min, *cmp_v);
                    r
                };
                let pass_ranges = {
                    let mut r = ranges;
                    r[range_index(value_id)] = (*cmp_v + 1, max);
                    r
                };

                n_processable(fail_ranges, workflow_key, rule_index + 1, workflows)
                    + n_processable(pass_ranges, target, 0, workflows)
            }
        }
        Rule::LessThan(value_id, cmp_v, target) => {
            let (min, max) = ranges[range_index(value_id)];

            if min >= *cmp_v {
                n_processable(ranges, workflow_key, rule_index + 1, workflows)
            } else if max < *cmp_v {
                n_processable(ranges, target, 0, workflows)
            } else {
                let fail_ranges = {
                    let mut r = ranges;
                    r[range_index(value_id)] = (*cmp_v, max);
                    r
                };
                let pass_ranges = {
                    let mut r = ranges;
                    r[range_index(value_id)] = (min, *cmp_v - 1);
                    r
                };

                n_processable(fail_ranges, workflow_key, rule_index + 1, workflows)
                    + n_processable(pass_ranges, target, 0, workflows)
            }
        }
        Rule::Accept => sum_ranges(&ranges),
        Rule::Reject => 0,
        Rule::Forward(target) => n_processable(ranges, target, 0, workflows),
    }
}

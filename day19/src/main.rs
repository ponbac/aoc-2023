use std::collections::HashMap;

use parse::{Part, Workflow};

use crate::parse::Rule;

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

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

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
}

fn process(part: &Part, workflow: &str, workflows: &HashMap<String, Workflow>) -> bool {
    let workflow = workflows.get(workflow).unwrap();

    for rule in &workflow.rules {
        match rule {
            Rule::GreaterThan(value_id, value, target) => {
                if part.get(value_id) <= *value {
                    continue;
                }

                if target == "A" {
                    return true;
                } else if target == "R" {
                    return false;
                }

                return process(part, target, workflows);
            }
            Rule::LessThan(value_id, value, target) => {
                if part.get(value_id) >= *value {
                    continue;
                }

                if target == "A" {
                    return true;
                } else if target == "R" {
                    return false;
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

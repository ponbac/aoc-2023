static EXAMPLE_INPUT: &str = r#"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;

fn main() {
    println!("\n-- Advent of Code 2023 - Day 25 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    todo!()
}

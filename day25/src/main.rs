use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use petgraph::{graph::UnGraph, stable_graph::NodeIndex, visit::Dfs};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

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

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let (graph, _) = parse_input(input);

    let edge_ids: Vec<_> = graph.edge_indices().collect();

    // Iterating over all combinations of three edges
    let solution_found = Arc::new(Mutex::new(false));

    println!("Number of edges: {}", edge_ids.len());
    edge_ids.par_iter().skip(23).for_each(|&edge1| {
        if *solution_found.lock().unwrap() {
            return;
        }
        println!("Working on edge {}", edge1.index());

        for &edge2 in &edge_ids {
            // println!(
            //     "Layer 2: {} - Working on edge {}",
            //     edge1.index(),
            //     edge2.index()
            // );
            if edge2 == edge1 {
                continue;
            }
            for &edge3 in &edge_ids {
                if edge3 == edge1 || edge3 == edge2 {
                    continue;
                }

                let mut graph = graph.clone();

                // Remove the edges
                let edges = [edge1, edge2, edge3];
                for &e in &edges {
                    graph.remove_edge(e);
                }

                // Check if the graph is now in two components
                let components = petgraph::algo::connected_components(&graph);
                if components == 2 {
                    // Calculate the product of the sizes of the components
                    let sizes_product: usize = calculate_component_sizes(&graph);
                    println!("Solution found! Sizes product: {}", sizes_product);
                    *solution_found.lock().unwrap() = true;
                    return;
                }
            }
        }
    });

    println!("No solution found that divides the graph into exactly two components.");
}

fn calculate_component_sizes(graph: &UnGraph<&str, ()>) -> usize {
    let mut visited = HashSet::new();
    let mut component_sizes = vec![];

    for node in graph.node_indices() {
        if !visited.contains(&node) {
            let mut dfs = Dfs::new(&graph, node);
            let mut size = 0;
            while let Some(nx) = dfs.next(&graph) {
                visited.insert(nx);
                size += 1;
            }
            component_sizes.push(size);
        }
    }

    component_sizes.iter().product()
}

fn parse_input(input: &str) -> (UnGraph<&str, ()>, HashMap<&str, NodeIndex>) {
    let mut graph = UnGraph::<&str, ()>::new_undirected();
    let mut node_map = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let node_name = parts[0];
        let connections = parts[1].split_whitespace();

        let node_index = *node_map
            .entry(node_name)
            .or_insert_with(|| graph.add_node(node_name));
        for connection in connections {
            let connection_index = *node_map
                .entry(connection)
                .or_insert_with(|| graph.add_node(connection));
            graph.add_edge(node_index, connection_index, ());
        }
    }

    (graph, node_map)
}

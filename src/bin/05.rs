use petgraph::{
    dot::{Config, Dot},
    graph::NodeIndex,
    Direction::Outgoing,
    Graph,
};
use std::collections::{HashMap, VecDeque};
use std::error::Error;

advent_of_code::solution!(5);

struct Problem {
    sequences: Vec<Vec<u32>>,
    g: Graph<u32, i32>,
    nodes: HashMap<u32, NodeIndex>,
}

fn parse_input(input: &str) -> Result<Problem, Box<dyn Error>> {
    let mut sequences: Vec<Vec<u32>> = Vec::new();
    let mut reading_sequences = false;
    let mut g = Graph::new();
    let mut nodes = HashMap::new();

    let lines = input.lines();

    for line in lines {
        let line = line.trim().to_string();
        if line.is_empty() {
            reading_sequences = true;
            if cfg!(debug_assertions) {
                println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
            }
            continue;
        }
        if !reading_sequences {
            if let Some((from, to)) = line.split_once('|') {
                let from_node: u32 = from.parse()?;
                let to_node: u32 = to.parse()?;
                let src = *nodes
                    .entry(from_node)
                    .or_insert_with(|| g.add_node(from_node));
                let dst = *nodes.entry(to_node).or_insert_with(|| g.add_node(to_node));
                g.add_edge(src, dst, 1);
            }
        } else {
            let sequence: Vec<u32> = line
                .split(',')
                .map(|n| n.parse().expect("Invalid node"))
                .collect();
            sequences.push(sequence);
        }
    }

    Ok(Problem {
        sequences,
        g,
        nodes,
    })
}

fn topological_sort(
    g: &Graph<u32, i32>,
    nodes: &HashMap<u32, NodeIndex>,
    subset: &[u32],
) -> Option<Vec<u32>> {
    let mut in_degree = subset
        .iter()
        .filter_map(|node| nodes.get(node).copied())
        .map(|idx| (idx, 0))
        .collect::<HashMap<_, _>>();

    for edge in g.edge_indices() {
        let (src, dst) = g.edge_endpoints(edge).unwrap();
        if in_degree.contains_key(&src) && in_degree.contains_key(&dst) {
            *in_degree.entry(dst).or_default() += 1;
        }
    }

    let mut queue = in_degree
        .iter()
        .filter_map(|(&idx, &deg)| if deg == 0 { Some(idx) } else { None })
        .collect::<VecDeque<_>>();

    let mut sorted = Vec::new();
    while let Some(node_idx) = queue.pop_front() {
        sorted.push(g[node_idx]);

        for neighbor in g.neighbors_directed(node_idx, Outgoing) {
            if let Some(deg) = in_degree.get_mut(&neighbor) {
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if sorted.len() == subset.len() {
        Some(sorted)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let p = parse_input(input).unwrap();
    for sequence in p.sequences {
        if let Some(sorted) = topological_sort(&p.g, &p.nodes, &sequence) {
            if sorted == sequence {
                sum += sorted[sorted.len() / 2];
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let p = parse_input(input).unwrap();
    for sequence in p.sequences {
        if let Some(sorted) = topological_sort(&p.g, &p.nodes, &sequence) {
            if sorted != sequence {
                sum += sorted[sorted.len() / 2];
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

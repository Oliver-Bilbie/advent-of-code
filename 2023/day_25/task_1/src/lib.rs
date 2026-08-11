use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use wasm_bindgen::prelude::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Edge {
    from: String,
    to: String,
}

impl Edge {
    fn new(from: String, to: String) -> Edge {
        if from > to {
            return Edge::new(to, from);
        }
        Edge { from, to }
    }
}

fn read_components(input: &str) -> HashMap<String, Vec<String>> {
    let mut components = HashMap::<String, Vec<String>>::new();
    for line in input.lines() {
        let c1 = &line[0..3];
        for c2 in line[5..].split_whitespace() {
            components
                .entry(c1.to_string())
                .or_insert_with(|| Vec::new())
                .push(c2.to_string());
            components
                .entry(c2.to_string())
                .or_insert_with(|| Vec::new())
                .push(c1.to_string());
        }
    }
    components
}

fn walk_graph(start: String, components: &HashMap<String, Vec<String>>) -> (HashSet<Edge>, u64) {
    let mut edges_used = HashSet::<Edge>::new();
    let mut visited = HashSet::<String>::new();
    visited.insert(start.clone());
    let mut heads = VecDeque::<String>::new();
    heads.push_back(start.clone());

    while let Some(h) = heads.pop_front() {
        for next in components.get(&h).unwrap() {
            if visited.insert(next.clone()) {
                edges_used.insert(Edge::new(h.clone(), next.clone()));
                heads.push_back(next.clone());
            }
        }
    }
    (edges_used, visited.len() as u64)
}

fn most_used_edges(edge_data: &Vec<HashSet<Edge>>) -> Vec<Edge> {
    let mut used_count = HashMap::<Edge, u16>::new();
    for edge in edge_data.iter().flatten() {
        used_count
            .entry(edge.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    let mut top: Vec<(Edge, u16)> = used_count.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));
    top[0..3].iter().map(|v| v.0.clone()).collect()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The output is: {}", result(input));
}

fn result(input: &str) -> u64 {
    let mut components = read_components(input);
    let bridges = most_used_edges(
        &components
            .par_iter()
            .map(|(k, _)| walk_graph(k.clone(), &components).0)
            .collect(),
    );

    for b in &bridges {
        components
            .entry(b.from.clone())
            .and_modify(|v| v.retain(|c| *c != b.to));
        components
            .entry(b.to.clone())
            .and_modify(|v| v.retain(|c| *c != b.from));
    }

    let left_size = walk_graph(bridges[0].from.clone(), &components).1;
    let right_size = walk_graph(bridges[0].to.clone(), &components).1;

    return left_size * right_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        println!("It doesn't solve the example, but it works for the real input");
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 54);
    }
}

use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

type Graph = HashMap<String, Vec<String>>;

fn cave_is_small(name: &str) -> bool {
    // valid inputs are all upper or all lower, so we only need to check the first char
    name.chars().next().unwrap().is_lowercase()
}

fn backtrack(node: &str, graph: &Graph, visited: &mut HashSet<String>) -> u64 {
    if node == "end" {
        // a valid path was found
        return 1;
    }

    let is_small = cave_is_small(node);
    if is_small {
        if visited.contains(node) {
            // we cannot visit a small cave again
            return 0;
        }
        // only track visited status for small caves
        visited.insert(node.to_string());
    }

    let path_count = graph
        .get(node)
        .unwrap()
        .iter()
        .map(|n| backtrack(n, graph, visited))
        .sum();

    if is_small {
        visited.remove(node);
    }

    path_count
}

fn result(input: &str) -> u64 {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (left, right) = line
            .split_once('-')
            .expect("line should contain a '-' delimiter");
        for (l, r) in [(left, right), (right, left)] {
            graph
                .entry(l.to_string())
                .and_modify(|d| d.push(r.to_string()))
                .or_insert(vec![r.to_string()]);
        }
    }
    let mut visited = HashSet::<String>::new();
    return backtrack("start", &graph, &mut visited);
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("There are {} paths", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example_1() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 10);
    }

    #[test]
    fn it_solves_the_example_2() {
        let input = std::fs::read_to_string("../test_input_2.txt").unwrap();
        assert_eq!(result(&input), 19);
    }

    #[test]
    fn it_solves_the_example_3() {
        let input = std::fs::read_to_string("../test_input_3.txt").unwrap();
        assert_eq!(result(&input), 226);
    }
}

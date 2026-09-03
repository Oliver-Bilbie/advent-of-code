use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

type Graph = HashMap<String, Vec<String>>;

fn cave_is_small(name: &str) -> bool {
    // valid inputs are all upper or all lower, so we only need to check the first char
    name.chars().next().unwrap().is_lowercase()
}

fn backtrack(node: &str, graph: &Graph, visited: &mut HashSet<String>, revisit_used: bool) -> u64 {
    if node == "end" {
        // a valid path was found
        return 1;
    }

    let is_small = cave_is_small(node);
    let mut is_revisit = false;
    if is_small {
        if visited.contains(node) {
            if revisit_used {
                // we cannot visit a small cave again
                return 0;
            }
            if node == "start" {
                // we may not return to the starting cave
                return 0;
            }
            is_revisit = true;
        } else {
            // only track visited status for small caves
            visited.insert(node.to_string());
        }
    }

    let path_count = graph
        .get(node)
        .unwrap()
        .iter()
        .map(|n| backtrack(n, graph, visited, revisit_used || is_revisit))
        .sum();

    if is_small && !is_revisit {
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
    return backtrack("start", &graph, &mut visited, false);
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("There are actually {} paths", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example_1() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 36);
    }

    #[test]
    fn it_solves_the_example_2() {
        let input = std::fs::read_to_string("../test_input_2.txt").unwrap();
        assert_eq!(result(&input), 103);
    }

    #[test]
    fn it_solves_the_example_3() {
        let input = std::fs::read_to_string("../test_input_3.txt").unwrap();
        assert_eq!(result(&input), 3509);
    }
}

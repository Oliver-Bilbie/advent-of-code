use aoc_utils::{
    direction::ALL_DIRECTIONS,
    graph::graph::{Edge, Graph},
    grid::Grid,
    position::Position,
};
use wasm_bindgen::prelude::*;

fn result(input: &str) -> u64 {
    let grid = Grid::from_str(input, |line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect()
    });
    let mut graph = Graph::<Position>::new();

    for source in grid.positions() {
        for dir in ALL_DIRECTIONS {
            if let Some(destination) = dir.travel_with_bounds(&source, grid.size()) {
                let weight = *grid.get(&destination).unwrap();
                graph.add_edge(Edge {
                    source: source.clone(),
                    destination,
                    weight,
                });
            }
        }
    }

    if graph.dijkstra(Position { row: 0, column: 0 }).is_err() {
        panic!("could not compute distances");
    };

    // somehow the yuckiest bit is reading the distance from the graph!
    graph
        .get_node(&Position {
            row: grid.size().row - 1,
            column: grid.size().column - 1,
        })
        .unwrap()
        .min_distance
        .unwrap()
        .try_into()
        .unwrap()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The lowest risk is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 40);
    }
}

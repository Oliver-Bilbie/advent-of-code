use aoc_utils::{
    direction::*,
    graph::graph::{Edge, Graph},
    position::*,
};
use std::collections::HashSet;

fn find_minimum_distance(graph: &mut Graph<Position>, boundary: &Position) -> Option<u128> {
    let start = Position { row: 0, column: 0 };
    let finish = boundary - Position { row: 1, column: 1 };

    graph.dijkstra(start).ok()?;

    graph.get_node_distance(&finish)
}

fn build_graph(input: &str, corrupted_bytes: usize, boundary: &Position) -> Graph<Position> {
    let mut graph = Graph::new();

    let corrupted_spaces: HashSet<Position> = input
        .lines()
        .take(corrupted_bytes)
        .map(|xy_str| {
            let (column, row) = xy_str.split_once(',').unwrap();
            Position {
                row: row.parse::<usize>().unwrap(),
                column: column.parse::<usize>().unwrap(),
            }
        })
        .collect();

    (0..boundary.row).into_iter().for_each(|row| {
        (0..boundary.column).into_iter().for_each(|column| {
            let source = Position { row, column };
            if !corrupted_spaces.contains(&source) {
                for direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if let Some(destination) = direction.travel_with_bounds(&source, &boundary) {
                        if !corrupted_spaces.contains(&destination) {
                            graph.add_edge(Edge {
                                source: source.clone(),
                                destination,
                                weight: 1,
                            });
                        }
                    }
                }
            }
        })
    });

    graph
}

fn solve_with_parameters(input: &str, corrupted_bytes: usize, boundary: Position) -> u128 {
    let mut graph = build_graph(&input, corrupted_bytes, &boundary);
    find_minimum_distance(&mut graph, &boundary).expect("no path was found")
}

pub fn solve(input: &str) -> String {
    let corrupted_bytes = 1024;
    let boundary = Position {
        row: 71,
        column: 71,
    };
    let min_distance = solve_with_parameters(&input, corrupted_bytes, boundary);

    format!("The minimum distance is: {}", min_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let corrupted_bytes = 12;
        let boundary = Position { row: 7, column: 7 };
        let actual_solution = solve_with_parameters(&input, corrupted_bytes, boundary);
        let expected_solution = 22;
        assert_eq!(actual_solution, expected_solution);
    }
}

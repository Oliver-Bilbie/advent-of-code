use aoc_utils::{
    direction::*,
    graph::graph::{Edge, Graph},
    position::*,
};
use std::collections::HashSet;
use std::fs;

fn find_minimum_distance(graph: &mut Graph<Position>, boundary: &Position) -> Option<u128> {
    let start = Position { row: 0, column: 0 };
    let finish = boundary - Position { row: 1, column: 1 };

    graph.dijkstra(start).ok()?;

    graph.get_node_distance(&finish)
}

fn build_graph(input_str: &str, corrupted_bytes: usize, boundary: &Position) -> Graph<Position> {
    let mut graph = Graph::new();

    let corrupted_spaces: HashSet<Position> = input_str
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

fn find_blocking_byte<'a>(input_str: &'a str, boundary: &Position) -> &'a str {
    let mut corrupted_bytes = 1;
    loop {
        let mut graph = build_graph(input_str, corrupted_bytes, boundary);
        if find_minimum_distance(&mut graph, &boundary).is_none() {
            break;
        }
        corrupted_bytes += 1;
    }

    input_str.lines().nth(corrupted_bytes - 1).unwrap()
}

fn main() {
    let input_str = fs::read_to_string("../input.txt").unwrap();
    let boundary = Position {
        row: 71,
        column: 71,
    };
    let blocking_byte = find_blocking_byte(&input_str, &boundary);
    println!("The first blocking byte is: {}", blocking_byte);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input_str = fs::read_to_string("../test_input.txt").unwrap();
        let boundary = Position { row: 7, column: 7 };
        let actual_byte = find_blocking_byte(&input_str, &boundary);

        let expected_byte = "6,1";
        assert_eq!(actual_byte, expected_byte);
    }
}

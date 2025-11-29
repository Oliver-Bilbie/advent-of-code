use aoc_utils::{
    direction::*,
    graph::graph::{Edge, Graph},
    position::*,
};
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Tile {
    position: Position,
    direction: Direction,
}

struct Maze {
    graph: Graph<Tile>,
    start: Position,
    finish: Position,
}

fn read_maze(input: &str) -> Maze {
    let mut graph = Graph::new();

    let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let boundary = Position {
        row: tiles.len(),
        column: tiles.first().unwrap().len(),
    };

    // Create a graph where the nodes have the dimensions of (row, column, entry_direction)
    // since nodes in the same physical position are reachable in different numbers of moves
    // depending on which direction they are approached from.
    for row in 0..boundary.row {
        for column in 0..boundary.column {
            for source_direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let source = Tile {
                    position: Position { row, column },
                    direction: source_direction.clone(),
                };

                for travel_direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if let Some(destination_position) =
                        travel_direction.travel_with_bounds(&Position { row, column }, &boundary)
                    {
                        if tiles[destination_position.row][destination_position.column] != '#' {
                            let destination = Tile {
                                position: destination_position,
                                direction: travel_direction.clone(),
                            };

                            let weight = if travel_direction == source_direction {
                                1 // Move forward
                            } else {
                                1001 // Turn and move
                            };

                            graph.add_edge(Edge {
                                source: source.clone(),
                                destination,
                                weight,
                            })
                        }
                    }
                }
            }
        }
    }

    let start = tiles
        .iter()
        .enumerate()
        .find_map(|(row, inner_vec)| {
            inner_vec
                .iter()
                .position(|&x| x == 'S')
                .map(|column| Position { row, column })
        })
        .unwrap();
    let finish = tiles
        .iter()
        .enumerate()
        .find_map(|(row, inner_vec)| {
            inner_vec
                .iter()
                .position(|&x| x == 'E')
                .map(|column| Position { row, column })
        })
        .unwrap();

    Maze {
        graph,
        start,
        finish,
    }
}

fn find_minimum_distance(maze: &mut Maze) -> Option<u128> {
    let start = Tile {
        position: maze.start.clone(),
        direction: Direction::Right,
    };

    maze.graph.dijkstra(start).unwrap();

    // Attempt to approach the end tile from all directions and take the
    // shortest distance
    let mut min_distance: Option<u128> = None;
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let finish = Tile {
            position: maze.finish.clone(),
            direction,
        };
        if let Some(distance) = maze.graph.get_node_distance(&finish) {
            match min_distance {
                Some(current_min) => {
                    if distance < current_min {
                        min_distance = Some(distance)
                    }
                }
                None => min_distance = Some(distance),
            }
        }
    }

    min_distance
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut maze = read_maze(&input);
    let min_distance = find_minimum_distance(&mut maze);
    format!("The minimum distance is: {}", min_distance.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The minimum distance is: 7036";
        assert_eq!(actual_solution, expected_solution);
    }
}

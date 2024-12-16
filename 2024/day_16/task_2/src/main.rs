use aoc_utils::{
    direction::*,
    graph::graph::{Edge, Graph},
    position::*,
};
use std::collections::HashSet;
use std::fs;

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

fn read_maze(input_file: &str) -> Maze {
    let mut graph = Graph::new();

    let tiles: Vec<Vec<char>> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

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

fn get_paths(maze: &Maze, finish: &Tile) -> Vec<Tile> {
    let mut route: Vec<Tile> = vec![finish.clone()];
    let next_tiles = &maze.graph.get_node(&finish).unwrap().previous_location;

    if next_tiles.len() == 0 {
        return route;
    }

    for tile in next_tiles {
        route.append(&mut get_paths(&maze, tile));
    }

    route
}

fn count_best_tiles(maze: &mut Maze) -> u64 {
    let start = Tile {
        position: maze.start.clone(),
        direction: Direction::Right,
    };

    maze.graph.dijkstra(start).unwrap();

    // Attempt to approach the end tile from all directions and take the
    // shortest distance
    let mut min_distance: Option<u128> = None;
    let mut finish: Option<Tile> = None;
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let possible_finish = Tile {
            position: maze.finish.clone(),
            direction,
        };
        if let Some(distance) = maze.graph.get_node_distance(&possible_finish) {
            match min_distance {
                Some(current_min) => {
                    if distance < current_min {
                        min_distance = Some(distance);
                        finish = Some(possible_finish);
                    }
                }
                None => {
                    min_distance = Some(distance);
                    finish = Some(possible_finish);
                }
            }
        }
    }

    let best_path_nodes = maze.graph.get_path_nodes(&finish.unwrap()).unwrap();
    // We now need to de-duplicate the nodes with respect to physical position.
    let positions: HashSet<Position> =
        HashSet::from_iter(best_path_nodes.iter().map(|tile| tile.position.clone()));

    positions.len() as u64
}

fn main() {
    let mut maze = read_maze("../input.txt");
    let best_tiles = count_best_tiles(&mut maze);
    println!("The number of best tiles is: {}", best_tiles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let mut maze = read_maze("../test_input.txt");
        let actual_count = count_best_tiles(&mut maze);
        let expected_count = 45;
        assert_eq!(actual_count, expected_count);
    }
}
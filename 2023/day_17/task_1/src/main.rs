use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn travel(&self, position: &Position, boundary: &Position) -> Option<Position> {
        match self {
            Direction::Up => {
                if position.row > 0 {
                    Some(Position {
                        row: position.row - 1,
                        column: position.column,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if position.row + 1 < boundary.row {
                    Some(Position {
                        row: position.row + 1,
                        column: position.column,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if position.column > 0 {
                    Some(Position {
                        row: position.row,
                        column: position.column - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if position.column + 1 < boundary.column {
                    Some(Position {
                        row: position.row,
                        column: position.column + 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Edge {
    target: usize,
    weight: u8,
}

#[derive(Clone, Debug)]
struct Node {
    min_distance: Option<u128>,
    visited: bool,
    edges: Vec<Edge>,
}

fn read_grid(source_file: &str) -> Vec<Vec<u8>> {
    let input = fs::read_to_string(source_file).unwrap();
    input
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn get_id(
    position: &Position,
    direction: &Direction,
    direction_count: u8,
    boundary: &Position,
) -> usize {
    let direction_multiplier = match direction {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    };
    (direction_count as usize)
        + 3 * direction_multiplier
        + 12 * position.column
        + 12 * boundary.column * position.row
}

fn get_edge(
    grid: &Vec<Vec<u8>>,
    boundary: &Position,
    position: &Position,
    direction: &Direction,
    mut direction_count: u8,
    travel_direction: &Direction,
) -> Option<Edge> {
    match travel_direction.travel(position, boundary) {
        Some(target) => {
            if travel_direction == direction {
                if direction_count == 3 {
                    return None;
                }
                direction_count += 1;
            } else {
                direction_count = 1;
            }
            Some(Edge {
                target: get_id(&target, travel_direction, direction_count, boundary),
                weight: grid[target.row][target.column],
            })
        }
        None => None,
    }
}

fn get_edges(
    grid: &Vec<Vec<u8>>,
    boundary: &Position,
    position: &Position,
    direction: &Direction,
    direction_count: u8,
) -> Vec<Edge> {
    let mut edges = vec![];

    let valid_directions = match direction {
        Direction::Up => [Direction::Up, Direction::Left, Direction::Right],
        Direction::Down => [Direction::Down, Direction::Left, Direction::Right],
        Direction::Left => [Direction::Left, Direction::Up, Direction::Down],
        Direction::Right => [Direction::Right, Direction::Up, Direction::Down],
    };

    for travel_direction in valid_directions {
        if let Some(edge) = get_edge(
            grid,
            boundary,
            position,
            direction,
            direction_count,
            &travel_direction,
        ) {
            edges.push(edge);
        }
    }

    edges
}

fn read_graph(source_file: &str) -> (HashMap<usize, Node>, Position) {
    let grid = read_grid(source_file);
    let boundary = Position {
        row: grid.len(),
        column: grid.first().unwrap().len(),
    };

    let mut graph = HashMap::new();

    for (row, line) in grid.iter().enumerate() {
        for (column, _) in line.iter().enumerate() {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                for direction_count in 1..4 {
                    graph.insert(
                        get_id(
                            &Position { row, column },
                            &direction,
                            direction_count,
                            &boundary,
                        ),
                        Node {
                            min_distance: None,
                            visited: false,
                            edges: get_edges(
                                &grid,
                                &boundary,
                                &Position { row, column },
                                &direction,
                                direction_count,
                            ),
                        },
                    );
                }
            }
        }
    }

    graph
        .get_mut(&get_id(
            &Position { row: 0, column: 0 },
            &Direction::Right,
            1,
            &boundary,
        ))
        .unwrap()
        .min_distance = Some(0);

    (graph, boundary)
}

// fn find_closest_unvisited_node(graph: &HashMap<usize, Node>) -> Option<usize> {
//     graph
//         .iter()
//         .filter(|(_, node)| !node.visited) // Only consider unvisited nodes
//         .filter_map(|(&id, node)| node.min_distance.map(|dist| (id, dist))) // Only consider nodes with a distance set
//         .min_by_key(|&(_, dist)| dist) // Find the node with the smallest distance
//         .map(|(id, _)| id) // Return the node ID
// }

// fn dijkstra(node_id: usize, graph: &mut HashMap<usize, Node>, boundary: &Position) {
//     // Mark the current node as visited and update its neighbors
//     {
//         let edges = {
//             let node = graph
//                 .get_mut(&node_id)
//                 .expect("the node should exist in the graph");
//             node.visited = true;
//             let current_distance = node.min_distance.unwrap();

//             // Collect edges for processing outside the mutable borrow scope
//             node.edges
//                 .iter()
//                 .map(|edge| (edge.target, current_distance + edge.weight as u128))
//                 .collect::<Vec<_>>()
//         };

//         // Update the distances of neighbors
//         for (target, new_distance) in edges {
//             if let Some(neighbor) = graph.get_mut(&target) {
//                 if let Some(current_distance) = neighbor.min_distance {
//                     // Update only if the new distance is smaller
//                     if new_distance < current_distance {
//                         neighbor.min_distance = Some(new_distance);
//                     }
//                 } else {
//                     // If no distance is set, set the new distance
//                     neighbor.min_distance = Some(new_distance);
//                 }
//             }
//         }
//     }

//     // Recursively apply Dijkstra's algorithm on the closest unvisited node
//     if let Some(next_node_id) = find_closest_unvisited_node(graph) {
//         dijkstra(next_node_id, graph, boundary);
//     }
// }

fn dijkstra(start_node_id: usize, graph: &mut HashMap<usize, Node>) {
    // Priority queue for the Dijkstra's algorithm (min-heap)
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0u128, start_node_id))); // (distance, node_id)

    while let Some(Reverse((current_distance, node_id))) = priority_queue.pop() {
        // Skip if the node is already visited
        if graph.get(&node_id).unwrap().visited {
            continue;
        }

        // Mark the current node as visited
        let edges = {
            let node = graph.get_mut(&node_id).unwrap();
            node.visited = true;

            // Collect edges to avoid mutable borrow issues
            node.edges.clone()
        };

        // Update distances for all neighbors
        for edge in edges {
            let neighbor = graph.get_mut(&edge.target).unwrap();
            if !neighbor.visited {
                let new_distance = current_distance + edge.weight as u128;
                if let Some(existing_distance) = neighbor.min_distance {
                    if new_distance < existing_distance {
                        neighbor.min_distance = Some(new_distance);
                        priority_queue.push(Reverse((new_distance, edge.target)));
                    }
                } else {
                    neighbor.min_distance = Some(new_distance);
                    priority_queue.push(Reverse((new_distance, edge.target)));
                }
            }
        }
    }
}

fn main() {
    let (mut graph, boundary) = read_graph("../input.txt");

    // TODO: Add start node with 0 travel
    let start_node_id = get_id(
        &Position { row: 0, column: 0 },
        &Direction::Right,
        1,
        &boundary,
    );

    dijkstra(start_node_id, &mut graph);

    let mut min_distance = None;
    for direction in [Direction::Down, Direction::Right] {
        for direction_count in 1..4 {
            if let Some(distance) = graph
                .get(&get_id(
                    &Position {
                        row: boundary.row - 1,
                        column: boundary.column - 1,
                    },
                    &direction,
                    direction_count,
                    &boundary,
                ))
                .unwrap()
                .min_distance
            {
                min_distance = match min_distance {
                    Some(current_min) => match distance < current_min {
                        true => Some(distance),
                        false => Some(current_min),
                    },
                    None => Some(distance),
                };
            };
        }
    }

    println!("The minimum distance is: {} tiles", min_distance.unwrap());
}

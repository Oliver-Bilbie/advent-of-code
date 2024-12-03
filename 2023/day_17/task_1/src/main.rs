use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
struct Blocks {
    grid: Vec<Vec<u8>>,
    boundary: Position,
}

#[derive(Eq, PartialEq)]
struct PriorityQueueEntry {
    node_id: usize,
    distance: u128,
}

impl Ord for PriorityQueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // The BinaryHeap returns the largest value first. Since we want the
        // smallest distance value first, we reverse the direction of comparison.
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for PriorityQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_blocks(source_file: &str) -> Blocks {
    let grid: Vec<Vec<u8>> = fs::read_to_string(source_file)
        .unwrap()
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let boundary = Position {
        row: grid.len(),
        column: grid.first().unwrap().len(),
    };

    Blocks { grid, boundary }
}

fn get_id(
    position: &Position,
    direction: &Direction,
    direction_count: u8,
    boundary: &Position,
) -> usize {
    let max_moves = 3;
    let direction_multiplier = match direction {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    };
    (direction_count as usize)
        + (max_moves + 1) * direction_multiplier
        + (max_moves + 1) * 4 * position.column
        + (max_moves + 1) * 4 * boundary.column * position.row
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

fn read_graph(blocks: &Blocks) -> HashMap<usize, Node> {
    let mut graph = HashMap::new();

    for (row, line) in blocks.grid.iter().enumerate() {
        for (column, _) in line.iter().enumerate() {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                for direction_count in 0..4 {
                    graph.insert(
                        get_id(
                            &Position { row, column },
                            &direction,
                            direction_count,
                            &blocks.boundary,
                        ),
                        Node {
                            min_distance: None,
                            visited: false,
                            edges: get_edges(
                                &blocks.grid,
                                &blocks.boundary,
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
}

fn dijkstra(start_node_id: usize, graph: &mut HashMap<usize, Node>) {
    // Initialize start node
    graph.get_mut(&start_node_id).unwrap().min_distance = Some(0);

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(PriorityQueueEntry {
        node_id: start_node_id,
        distance: 0,
    });

    while let Some(PriorityQueueEntry {
        node_id,
        distance: current_distance,
    }) = priority_queue.pop()
    {
        // Skip if the node is already visited
        if graph.get(&node_id).unwrap().visited {
            continue;
        }

        // Visit the node and read its connected edges
        let edges = {
            let node = graph.get_mut(&node_id).unwrap();
            node.visited = true;

            node.edges.clone()
        };

        // Update distances for all neighbors
        for edge in edges {
            let neighbor = graph.get_mut(&edge.target).unwrap();
            if !neighbor.visited {
                let new_distance = current_distance + edge.weight as u128;
                match neighbor.min_distance {
                    Some(current_distance) => {
                        if new_distance < current_distance {
                            neighbor.min_distance = Some(new_distance);
                            priority_queue.push(PriorityQueueEntry {
                                node_id: edge.target,
                                distance: new_distance,
                            })
                        }
                    }
                    None => {
                        neighbor.min_distance = Some(new_distance);
                        priority_queue.push(PriorityQueueEntry {
                            node_id: edge.target,
                            distance: new_distance,
                        })
                    }
                }
            }
        }
    }
}

fn minimum_distance(blocks: &Blocks) -> Option<u128> {
    let mut min_distance = None;

    for start_node_id in [
        get_id(
            &Position { row: 0, column: 0 },
            &Direction::Right,
            0,
            &blocks.boundary,
        ),
        get_id(
            &Position { row: 0, column: 0 },
            &Direction::Down,
            0,
            &blocks.boundary,
        ),
    ] {
        let mut graph = read_graph(blocks);
        dijkstra(start_node_id, &mut graph);

        for direction in [Direction::Down, Direction::Right] {
            for direction_count in 1..4 {
                if let Some(distance) = graph
                    .get(&get_id(
                        &Position {
                            row: blocks.boundary.row - 1,
                            column: blocks.boundary.column - 1,
                        },
                        &direction,
                        direction_count,
                        &blocks.boundary,
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
    }

    min_distance
}

fn main() {
    let blocks = read_blocks("../input.txt");

    println!(
        "The minimum distance is: {} tiles",
        minimum_distance(&blocks).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_example_grid() {
        let actual_blocks = read_blocks("../test_input.txt");
        let expected_grid = vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ];
        assert_eq!(actual_blocks.grid, expected_grid);
    }

    #[test]
    fn it_reads_example_boundary() {
        let actual_blocks = read_blocks("../test_input.txt");
        let expected_boundary = Position {
            row: 13,
            column: 13,
        };
        assert_eq!(actual_blocks.boundary, expected_boundary);
    }

    #[test]
    fn it_solves_the_example() {
        let blocks = read_blocks("../test_input.txt");
        let actual_min_distance = minimum_distance(&blocks);
        let expected_min_distance = Some(102);
        assert_eq!(actual_min_distance, expected_min_distance);
    }
}

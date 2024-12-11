use std::fs;
use std::ops::Add;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Position {
    row: i16,
    column: i16,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl Position {
    pub fn from_usize(row: usize, column: usize) -> Self {
        Self {
            row: row as i16,
            column: column as i16,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn travel(&self, position: &Position, boundary: &Position) -> Option<Position> {
        match self {
            Direction::North if position.row > 0 => Some(Position {
                row: position.row - 1,
                column: position.column,
            }),
            Direction::East if position.column + 1 < boundary.column => Some(Position {
                row: position.row,
                column: position.column + 1,
            }),
            Direction::South if position.row + 1 < boundary.row => Some(Position {
                row: position.row + 1,
                column: position.column,
            }),
            Direction::West if position.column > 0 => Some(Position {
                row: position.row,
                column: position.column - 1,
            }),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Node {
    position: Position,
    height: u8,
    visited: bool,
    neighbours: Vec<usize>,
}

impl Node {
    fn new(position: Position, height: u8) -> Self {
        Self {
            position,
            height,
            visited: false,
            neighbours: vec![],
        }
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<usize>>,
    nodes: Vec<Node>,
    boundary: Position,
}

impl Map {
    fn get_node_index(&self, position: &Position) -> Option<usize> {
        let row = usize::try_from(position.row).ok()?;
        let column = usize::try_from(position.column).ok()?;
        Some(*self.tiles.get(row)?.get(column)?)
    }

    fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    fn get_node_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }
}

fn read_heights(input_file: &str) -> Map {
    let mut nodes = vec![];
    let mut tiles = vec![];

    fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            let mut row_indices = vec![];
            line.chars().enumerate().for_each(|(column, c)| {
                let node = Node::new(
                    Position::from_usize(row, column),
                    c.to_digit(10).unwrap() as u8,
                );
                let index = nodes.len();
                nodes.push(node);
                row_indices.push(index);
            });
            tiles.push(row_indices);
        });

    let boundary = Position::from_usize(tiles.len(), tiles.iter().next().unwrap().len());
    Map {
        tiles,
        nodes,
        boundary,
    }
}

fn find_neighbours(map: &mut Map) {
    for node_index in 0..map.nodes.len() {
        let node = &map.nodes[node_index];
        let mut neighbours = vec![];

        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if let Some(position) = direction.travel(&node.position, &map.boundary) {
                if let Some(neighbour_index) = map.get_node_index(&position) {
                    let neighbour = &map.nodes[neighbour_index];
                    if neighbour.height == node.height + 1 {
                        neighbours.push(neighbour_index);
                    }
                }
            }
        }

        map.get_node_mut(node_index).unwrap().neighbours = neighbours;
    }
}

fn read_map(input_file: &str) -> Map {
    let mut map = read_heights(input_file);
    find_neighbours(&mut map);
    map
}

fn count_routes(map: Map) -> u64 {
    let trail_heads: Vec<usize> = map
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(id, node)| if node.height == 0 { Some(id) } else { None })
        .collect();

    trail_heads
        .iter()
        .map(|start_node| {
            let map = map.clone();

            // Stack to store current paths being explored
            let mut stack = vec![vec![*start_node]];
            let mut distinct_paths = 0;

            while let Some(path) = stack.pop() {
                let current_node_id = *path.last().unwrap();

                let current_node = map.get_node(current_node_id).unwrap();
                if current_node.height == 9 {
                    distinct_paths += 1;
                    continue;
                }

                let neighbors_to_visit: Vec<usize> = current_node
                    .neighbours
                    .iter()
                    .filter_map(|&neighbour_id| {
                        let neighbour = map.get_node(neighbour_id).unwrap();
                        if !path.contains(&neighbour_id)
                            && neighbour.height == current_node.height + 1
                        {
                            Some(neighbour_id)
                        } else {
                            None
                        }
                    })
                    .collect();

                // Push new paths to the stack
                for neighbour_id in neighbors_to_visit {
                    let mut new_path = path.clone();
                    new_path.push(neighbour_id);
                    stack.push(new_path);
                }
            }

            distinct_paths
        })
        .sum()
}
fn main() {
    let map = read_map("../input.txt");
    let route_count = count_routes(map);
    println!("The route count is: {}", route_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let map = read_map("../test_input.txt");
        let actual_count = count_routes(map);
        let expected_count = 81;
        assert_eq!(actual_count, expected_count);
    }
}

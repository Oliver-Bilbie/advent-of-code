use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Debug)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Invalid tile character"),
        }
    }
}

impl Direction {
    fn travel(&self, position: &Position, boundary: &Position) -> Option<Position> {
        match self {
            Direction::North => {
                if position.row > 0 {
                    Some(Position {
                        row: position.row - 1,
                        column: position.column,
                    })
                } else {
                    None
                }
            }
            Direction::South => {
                if position.row + 1 < boundary.row {
                    Some(Position {
                        row: position.row + 1,
                        column: position.column,
                    })
                } else {
                    None
                }
            }
            Direction::West => {
                if position.column > 0 {
                    Some(Position {
                        row: position.row,
                        column: position.column - 1,
                    })
                } else {
                    None
                }
            }
            Direction::East => {
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

#[derive(PartialEq, Clone)]
enum Tile {
    Robot,
    Box,
    Empty,
    Wall,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '@' => Tile::Robot,
            'O' => Tile::Box,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => panic!("Invalid tile character"),
        }
    }
}

struct Warehouse {
    robot_position: Position,
    directions: VecDeque<Direction>,
    tiles: Vec<Vec<Tile>>,
    boundary: Position,
}

impl Warehouse {
    fn get_tile(&self, position: &Position) -> Option<&Tile> {
        let row_index = usize::try_from(position.row).ok()?;
        let column_index = usize::try_from(position.column).ok()?;

        let row: &Vec<Tile> = self.tiles.get(row_index)?;
        let tile = row.get(column_index)?;

        Some(tile)
    }
}

fn read_warehouse(input: &str) -> Warehouse {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| Tile::from_char(c)).collect())
        .collect();

    let robot_position = input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .find_map(|(row, line)| {
            line.chars()
                .position(|c| c == '@')
                .map(|column| Position { row, column })
        })
        .unwrap();

    let directions = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .flat_map(|line| {
            line.chars()
                .map(|c| Direction::from_char(c))
                .collect::<Vec<Direction>>()
        })
        .collect();

    let boundary = Position {
        row: tiles.len(),
        column: tiles.first().unwrap().len(),
    };

    Warehouse {
        tiles,
        robot_position,
        directions,
        boundary,
    }
}

fn move_robot(warehouse: &mut Warehouse) -> Option<()> {
    let direction = warehouse.directions.pop_front()?;

    let mut current_position = warehouse.robot_position.clone();
    let mut tiles_to_shift = vec![current_position.clone()];

    // Find the tiles to be shifted by the robot
    loop {
        let next_position = direction.travel(&current_position, &warehouse.boundary)?;
        match warehouse.get_tile(&next_position)? {
            Tile::Box => {
                tiles_to_shift.push(next_position.clone());
                current_position = next_position;
            }
            Tile::Wall => {
                return None;
            }
            Tile::Empty => {
                break;
            }
            Tile::Robot => unreachable!(),
        }
    }

    // Shift the tiles
    for current_position in tiles_to_shift.iter().rev() {
        let tile = warehouse.get_tile(&current_position)?.clone();
        let new_position = direction.travel(&current_position, &warehouse.boundary)?;
        warehouse.tiles[new_position.row][new_position.column] = tile;
    }

    // Finish shifting the robot
    warehouse.tiles[warehouse.robot_position.row][warehouse.robot_position.column] = Tile::Empty;
    warehouse.robot_position = direction.travel(&warehouse.robot_position, &warehouse.boundary)?;

    Some(())
}

fn sum_all_gps(warehouse: &Warehouse) -> u64 {
    warehouse
        .tiles
        .iter()
        .enumerate()
        .map(|(row, content)| {
            content
                .iter()
                .enumerate()
                .map(|(column, tile)| {
                    if *tile == Tile::Box {
                        (100 * row + column) as u64
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut warehouse = read_warehouse(&input);
    while warehouse.directions.len() > 0 {
        move_robot(&mut warehouse);
    }
    let gps_sum = sum_all_gps(&warehouse);

    format!("The GPS sum is: {}", gps_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_small_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The GPS sum is: 2028";
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn it_solves_the_large_example() {
        let input = std::fs::read_to_string("../test_input_2.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The GPS sum is: 10092";
        assert_eq!(actual_solution, expected_solution);
    }
}

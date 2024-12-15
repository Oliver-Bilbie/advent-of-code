use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
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
    BoxLeft,
    BoxRight,
    Empty,
    Wall,
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

fn read_warehouse(input_file: &str) -> Warehouse {
    let input_str = fs::read_to_string(input_file).unwrap();

    let tiles: Vec<Vec<Tile>> = input_str
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '@' => vec![Tile::Robot, Tile::Empty],
                    'O' => vec![Tile::BoxLeft, Tile::BoxRight],
                    '.' => vec![Tile::Empty, Tile::Empty],
                    '#' => vec![Tile::Wall, Tile::Wall],
                    _ => panic!("Invalid tile character"),
                })
                .collect()
        })
        .collect();

    let robot_position = input_str
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .find_map(|(row, line)| {
            line.chars().position(|c| c == '@').map(|column| Position {
                row,
                column: 2 * column,
            })
        })
        .unwrap();

    let directions = input_str
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

    let mut current_positions = vec![warehouse.robot_position.clone()];
    let mut tiles_to_shift = vec![warehouse.robot_position.clone()];

    // Find the tiles to be shifted by the robot
    loop {
        let next_positions: Vec<Position> = current_positions
            .iter()
            .filter_map(|current_position| direction.travel(&current_position, &warehouse.boundary))
            .collect();
        current_positions = vec![];

        if next_positions.len() == 0 {
            break;
        }

        for next_position in next_positions {
            match warehouse.get_tile(&next_position)? {
                Tile::BoxLeft => {
                    tiles_to_shift.push(next_position.clone());
                    current_positions.push(next_position.clone());

                    if direction == Direction::North || direction == Direction::South {
                        let box_right =
                            Direction::East.travel(&next_position, &warehouse.boundary)?;
                        tiles_to_shift.push(box_right.clone());
                        current_positions.push(box_right);
                    }
                }
                Tile::BoxRight => {
                    tiles_to_shift.push(next_position.clone());
                    current_positions.push(next_position.clone());

                    if direction == Direction::North || direction == Direction::South {
                        let box_left =
                            Direction::West.travel(&next_position, &warehouse.boundary)?;
                        tiles_to_shift.push(box_left.clone());
                        current_positions.push(box_left);
                    }
                }
                Tile::Wall => {
                    return None;
                }
                Tile::Empty => {}
                Tile::Robot => unreachable!(),
            }
        }
    }

    // Remove duplicate tiles
    let mut set = HashSet::new();
    tiles_to_shift.retain(|x| set.insert((*x).clone()));

    // Shift the tiles
    for current_position in tiles_to_shift.iter().rev() {
        let tile = warehouse.get_tile(&current_position)?.clone();
        let new_position = direction.travel(&current_position, &warehouse.boundary)?;
        warehouse.tiles[new_position.row][new_position.column] = tile;
        warehouse.tiles[current_position.row][current_position.column] = Tile::Empty;
    }

    // Finish shifting the robot
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
                    if *tile == Tile::BoxLeft {
                        (100 * row + column) as u64
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

fn main() {
    let mut warehouse = read_warehouse("../input.txt");
    while warehouse.directions.len() > 0 {
        move_robot(&mut warehouse);
    }
    let gps_sum = sum_all_gps(&warehouse);

    println!("The GPS sum is: {}", gps_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_large_example() {
        let mut warehouse = read_warehouse("../test_input_2.txt");
        while warehouse.directions.len() > 0 {
            move_robot(&mut warehouse);
        }
        let actual_gps_sum = sum_all_gps(&warehouse);
        let expected_gps_sum = 9021;
        assert_eq!(actual_gps_sum, expected_gps_sum);
    }
}

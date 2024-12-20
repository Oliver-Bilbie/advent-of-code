use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Add, Sub};

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

impl<'a, 'b> Add<&'b Position> for &'a Position {
    type Output = Position;

    fn add(self, other: &'b Position) -> Position {
        Position {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl<'a, 'b> Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, other: &'b Position) -> Position {
        Position {
            row: self.row - other.row,
            column: self.column - other.column,
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

fn read_antennas(input_file: &str) -> (HashMap<char, Vec<Position>>, Position) {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    let input_str = fs::read_to_string(input_file).unwrap();

    input_str.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(column, value)| {
            if value != '.' {
                let position = Position::from_usize(row, column);
                antennas
                    .entry(value)
                    .and_modify(|positions| positions.push(position.clone()))
                    .or_insert(vec![position]);
            }
        })
    });

    let boundary = Position::from_usize(
        input_str.lines().count(),
        input_str.lines().next().unwrap().chars().count(),
    );

    (antennas, boundary)
}

fn find_antinodes(antenna_positions: &[Position], boundary: &Position) -> Vec<Position> {
    antenna_positions
        .iter()
        .enumerate()
        .flat_map(|(i, first_antenna)| {
            antenna_positions[i + 1..]
                .iter()
                .flat_map(move |second_antenna| {
                    let spatial_freq = second_antenna - first_antenna;
                    let mut antinodes = vec![];
                    let mut antinode_position = second_antenna.clone();

                    // Find increasing cases
                    while is_in_bounds(&antinode_position, boundary) {
                        antinodes.push(antinode_position.clone());
                        antinode_position = antinode_position + spatial_freq.clone();
                    }

                    // Find decreasing cases
                    antinode_position = first_antenna.clone();
                    while is_in_bounds(&antinode_position, boundary) {
                        antinodes.push(antinode_position.clone());
                        antinode_position = antinode_position - spatial_freq.clone();
                    }

                    antinodes
                })
        })
        .collect()
}

fn is_in_bounds(position: &Position, boundary: &Position) -> bool {
    if position.row < 0 || position.row >= boundary.row {
        return false;
    }
    if position.column < 0 || position.column >= boundary.column {
        return false;
    }

    true
}

fn count_antinode_positions(antennas: &HashMap<char, Vec<Position>>, boundary: &Position) -> u64 {
    antennas
        .iter()
        .flat_map(|(_, positions)| find_antinodes(positions, boundary))
        .collect::<HashSet<Position>>()
        .len() as u64
}

fn main() {
    let (antennas, boundary) = read_antennas("../input.txt");
    let antinode_count = count_antinode_positions(&antennas, &boundary);
    println!("There are {} unique antinode positions", antinode_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_antennas() {
        let (actual_antennas, actual_boundary) = read_antennas("../test_input.txt");
        let expected_antennas: HashMap<char, Vec<Position>> = HashMap::from([
            (
                '0',
                vec![
                    Position { row: 1, column: 8 },
                    Position { row: 2, column: 5 },
                    Position { row: 3, column: 7 },
                    Position { row: 4, column: 4 },
                ],
            ),
            (
                'A',
                vec![
                    Position { row: 5, column: 6 },
                    Position { row: 8, column: 8 },
                    Position { row: 9, column: 9 },
                ],
            ),
        ]);
        let expected_boundary = Position {
            row: 12,
            column: 12,
        };

        assert_eq!(actual_antennas, expected_antennas);
        assert_eq!(actual_boundary, expected_boundary);
    }

    #[test]
    fn it_solves_the_example() {
        let (antennas, boundary) = read_antennas("../test_input.txt");
        let actual_count = count_antinode_positions(&antennas, &boundary);
        let expected_count = 34;
        assert_eq!(actual_count, expected_count);
    }
}

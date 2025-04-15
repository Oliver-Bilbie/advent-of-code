use std::collections::{HashMap, HashSet};
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

fn read_antennas(input: &str) -> (HashMap<char, Vec<Position>>, Position) {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    input.lines().enumerate().for_each(|(row, line)| {
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
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
    );

    (antennas, boundary)
}

fn find_antinodes(antenna_positions: &[Position]) -> Vec<Position> {
    antenna_positions
        .iter()
        .enumerate()
        .flat_map(|(i, first_antenna)| {
            antenna_positions[i + 1..]
                .iter()
                .flat_map(move |second_antenna| {
                    vec![
                        first_antenna + first_antenna - second_antenna.clone(),
                        second_antenna + second_antenna - first_antenna.clone(),
                    ]
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
        .flat_map(|(_, positions)| find_antinodes(positions))
        .filter(|position| is_in_bounds(position, boundary))
        .collect::<HashSet<Position>>()
        .len() as u64
}

pub fn solve(input: &str) -> String {
    let (antennas, boundary) = read_antennas(&input);
    let antinode_count = count_antinode_positions(&antennas, &boundary);
    format!("There are {} unique antinode positions", antinode_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_antennas() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let (actual_antennas, actual_boundary) = read_antennas(&input);
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
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "There are 14 unique antinode positions";
        assert_eq!(actual_solution, expected_solution);
    }
}

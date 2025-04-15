use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
struct Tile {
    has_obstacle: bool,
    visited: bool,
}

#[derive(Eq, PartialEq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Position {
    row: i16,
    column: i16,
}

impl Position {
    pub fn from_usize(row: usize, column: usize) -> Option<Self> {
        let row = i16::try_from(row).ok()?;
        let column = i16::try_from(column).ok()?;
        Some(Self { row, column })
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Guard {
    position: Option<Position>,
    direction: Direction,
}

#[derive(Clone)]
struct Lab {
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
    history: HashSet<Guard>,
}

impl Lab {
    fn get_tile(&mut self, position: &Position) -> Option<&mut Tile> {
        let row_index = usize::try_from(position.row).ok()?;
        let column_index = usize::try_from(position.column).ok()?;

        let row: &mut Vec<Tile> = self.tiles.get_mut(row_index)?;
        let tile = row.get_mut(column_index)?;

        Some(tile)
    }
}

fn read_lab(input: &str) -> Lab {
    const OBSTACLE: char = '#';
    const GUARD: char = '^';
    const EMPTY: char = '.';

    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    OBSTACLE => Tile {
                        has_obstacle: true,
                        visited: false,
                    },
                    GUARD => Tile {
                        has_obstacle: false,
                        visited: true,
                    },
                    EMPTY => Tile {
                        has_obstacle: false,
                        visited: false,
                    },
                    _ => panic!("Invalid character found in the input"),
                })
                .collect()
        })
        .collect();

    let guard = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| {
            line.chars()
                .enumerate()
                .find_map(|(column, char)| match char {
                    GUARD => Some(Guard {
                        position: Position::from_usize(row, column),
                        direction: Direction::Up,
                    }),
                    _ => None,
                })
        })
        .unwrap();

    Lab {
        tiles,
        guard,
        history: HashSet::new(),
    }
}

fn move_guard(lab: &mut Lab) {
    if let Some(guard_position) = &lab.guard.position {
        let new_position = match &lab.guard.direction {
            Direction::Up => Position {
                row: guard_position.row - 1,
                column: guard_position.column,
            },
            Direction::Down => Position {
                row: guard_position.row + 1,
                column: guard_position.column,
            },
            Direction::Left => Position {
                row: guard_position.row,
                column: guard_position.column - 1,
            },
            Direction::Right => Position {
                row: guard_position.row,
                column: guard_position.column + 1,
            },
        };

        if let Some(new_tile) = lab.get_tile(&new_position) {
            if new_tile.has_obstacle {
                lab.guard.direction = lab.guard.direction.turn();
            } else {
                new_tile.visited = true;
                lab.guard.position = Some(new_position);
            }
        } else {
            lab.guard.position = None;
        };
    }
}

fn check_position(lab: &mut Lab) -> bool {
    while lab.guard.position.is_some() {
        lab.history.insert(lab.guard.clone());

        move_guard(lab);

        // Check if the guard has already been here
        if lab.history.contains(&lab.guard) {
            return true;
        }
    }

    false
}

fn add_obstacle(position: &Position, lab: &Lab) -> Result<Lab, String> {
    if *position == lab.guard.position.clone().unwrap() {
        return Err("The position contains the guard".to_string());
    }

    let mut new_lab = lab.clone();

    match new_lab.get_tile(&position) {
        Some(tile) => {
            if tile.has_obstacle {
                return Err("The position already contains an obstacle".to_string());
            } else {
                tile.has_obstacle = true;
            };
        }
        None => return Err("The position is out of bounds".to_string()),
    }

    Ok(new_lab)
}

fn find_guard_route(lab: &Lab) -> HashSet<Position> {
    let mut new_lab = lab.clone();
    let mut route = HashSet::new();

    while let Some(position) = &new_lab.guard.position {
        route.insert(position.clone());
        move_guard(&mut new_lab);
    }

    route
}

fn find_positions(lab: &Lab) -> u64 {
    // Find the route that the guard takes without placing any additional obstacles.
    // These are the only places we need to consider, since the guard will never reach
    // any obstacles placed elsewhere.
    let initial_route = find_guard_route(&lab);

    initial_route
        .par_iter()
        .filter(|position| {
            if let Ok(mut lab) = add_obstacle(&position, &lab) {
                check_position(&mut lab)
            } else {
                false
            }
        })
        .count() as u64
}

pub fn solve(input: &str) -> String {
    let lab = read_lab(&input);
    let possible_positions = find_positions(&lab);
    format!("There are {} possible positions", possible_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "There are 6 possible positions";
        assert_eq!(actual_solution, expected_solution);
    }
}

use std::fs;

struct Tile {
    has_obstacle: bool,
    visited: bool,
}

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

struct Guard {
    position: Option<Position>,
    direction: Direction,
}

struct Lab {
    tiles: Vec<Vec<Tile>>,
    guard: Guard,
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

fn read_lab(source_file: &str) -> Lab {
    const OBSTACLE: char = '#';
    const GUARD: char = '^';
    const EMPTY: char = '.';

    let tiles: Vec<Vec<Tile>> = fs::read_to_string(source_file)
        .unwrap()
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

    let guard = fs::read_to_string(source_file)
        .unwrap()
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

    Lab { tiles, guard }
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

fn count_visited(tiles: &Vec<Vec<Tile>>) -> u32 {
    tiles
        .iter()
        // Find the number of visited files per row
        .map(|row| row.iter().filter(|tile| tile.visited).count() as u32)
        .sum()
}

fn main() {
    let mut lab = read_lab("../input.txt");

    while lab.guard.position.is_some() {
        move_guard(&mut lab);
    }

    let visited_tiles = count_visited(&lab.tiles);
    println!("There are {} visited tiles", visited_tiles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let mut lab = read_lab("../test_input.txt");
        while lab.guard.position.is_some() {
            move_guard(&mut lab);
        }
        let actual_visited = count_visited(&lab.tiles);
        let expected_visited = 41;
        assert_eq!(actual_visited, expected_visited);
    }
}

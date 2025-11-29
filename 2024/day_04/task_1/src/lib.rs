use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Debug)]
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
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn travel(&self, position: &Position, boundary: &Position) -> Option<Position> {
        match self {
            Direction::Up => {
                if position.row > 0 {
                    return Some(Position {
                        row: position.row - 1,
                        column: position.column,
                    });
                }
            }
            Direction::Down => {
                if position.row + 1 < boundary.row {
                    return Some(Position {
                        row: position.row + 1,
                        column: position.column,
                    });
                }
            }
            Direction::Left => {
                if position.column > 0 {
                    return Some(Position {
                        row: position.row,
                        column: position.column - 1,
                    });
                }
            }
            Direction::Right => {
                if position.column + 1 < boundary.column {
                    return Some(Position {
                        row: position.row,
                        column: position.column + 1,
                    });
                }
            }
            Direction::UpLeft => {
                return Direction::Up
                    .travel(&Direction::Left.travel(position, boundary)?, boundary);
            }
            Direction::UpRight => {
                return Direction::Up
                    .travel(&Direction::Right.travel(position, boundary)?, boundary);
            }
            Direction::DownLeft => {
                return Direction::Down
                    .travel(&Direction::Left.travel(position, boundary)?, boundary);
            }
            Direction::DownRight => {
                return Direction::Down
                    .travel(&Direction::Right.travel(position, boundary)?, boundary);
            }
        }

        None
    }
}

fn is_word(start_position: &Position, direction: &Direction, grid: &Vec<Vec<char>>) -> bool {
    const LETTERS: [char; 3] = ['M', 'A', 'S'];

    let boundary = Position {
        row: grid.len(),
        column: grid.first().unwrap().len(),
    };
    let mut position = start_position.clone();

    for letter in LETTERS {
        match direction.travel(&position, &boundary) {
            None => return false,
            Some(next_position) => {
                if grid[next_position.row][next_position.column] != letter {
                    return false;
                }
                position = next_position;
            }
        }
    }

    true
}

fn x_iter<'a>(grid: &'a Vec<Vec<char>>) -> impl Iterator<Item = Position> + 'a {
    grid.iter().enumerate().flat_map(|(row, row_values)| {
        row_values
            .iter()
            .enumerate()
            .filter_map(move |(column, &value)| (value == 'X').then(|| Position { row, column }))
    })
}

fn count_words(grid: &Vec<Vec<char>>) -> u128 {
    const DIRECTIONS: [Direction; 8] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    x_iter(&grid)
        .map(|position| {
            DIRECTIONS
                .iter()
                .filter(|direction| is_word(&position, direction, grid))
                .count() as u128
        })
        .sum()
}

fn read_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let grid = read_grid(&input);
    let word_count = count_words(&grid);
    format!("There are {} occurances of XMAS", word_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> String {
        std::fs::read_to_string("../test_input.txt").unwrap()
    }

    #[test]
    fn it_reads_the_grid() {
        let input = read_test_input();
        let actual_grid = read_grid(&input);
        let expected_grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(actual_grid, expected_grid);
    }

    #[test]
    fn it_solves_the_example() {
        let input = read_test_input();
        let actual_solution = solve(&input);
        let expected_solution = "There are 18 occurances of XMAS";
        assert_eq!(actual_solution, expected_solution);
    }
}

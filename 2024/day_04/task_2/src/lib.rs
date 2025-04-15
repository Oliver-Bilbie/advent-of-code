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

fn is_cross(position: &Position, grid: &Vec<Vec<char>>) -> bool {
    const DIRECTIONS: [Direction; 4] = [
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    let boundary = Position {
        row: grid.len(),
        column: grid.first().unwrap().len(),
    };

    // Find the number of 'M' and 'S' characters in the diagonal corners
    let (m_count, s_count) = DIRECTIONS
        .iter()
        .filter_map(|direction| direction.travel(&position, &boundary))
        .map(|corner_position| grid[corner_position.row][corner_position.column])
        .fold((0, 0), |(m, s), letter| match letter {
            'M' => (m + 1, s),
            'S' => (m, s + 1),
            _ => (m, s),
        });

    if !(m_count == 2 && s_count == 2) {
        return false;
    }

    // Check whether diagonals contain the same character to remove MAM/SAS cases
    let upleft_pos = Direction::UpLeft.travel(&position, &boundary).unwrap();
    let downright_pos = Direction::DownRight.travel(&position, &boundary).unwrap();
    grid[upleft_pos.row][upleft_pos.column] != grid[downright_pos.row][downright_pos.column]
}

fn a_iter<'a>(grid: &'a Vec<Vec<char>>) -> impl Iterator<Item = Position> + 'a {
    grid.iter().enumerate().flat_map(|(row, row_values)| {
        row_values
            .iter()
            .enumerate()
            .filter_map(move |(column, &value)| (value == 'A').then(|| Position { row, column }))
    })
}

fn count_crosses(grid: &Vec<Vec<char>>) -> u128 {
    a_iter(&grid)
        .filter(|position| is_cross(position, grid))
        .count() as u128
}

fn read_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn solve(input: &str) -> String {
    let grid = read_grid(&input);
    let cross_count = count_crosses(&grid);
    format!("There are {} crosses", cross_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> String {
        std::fs::read_to_string("../test_input.txt").unwrap()
    }

    #[test]
    fn it_reads_the_grid() {
        let test_input = read_test_input();
        let actual_grid = read_grid(&test_input);
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
        let test_input = read_test_input();
        let actual_solution = solve(&test_input);
        let expected_solution = "There are 9 crosses";
        assert_eq!(actual_solution, expected_solution);
    }
}

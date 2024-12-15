use crate::position::Position;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn travel(&self, position: &Position) -> Position {
        match self {
            Direction::Up => Position {
                row: position.row - 1,
                column: position.column,
            },
            Direction::Down => Position {
                row: position.row + 1,
                column: position.column,
            },
            Direction::Left => Position {
                row: position.row,
                column: position.column - 1,
            },
            Direction::Right => Position {
                row: position.row,
                column: position.column + 1,
            },
        }
    }

    pub fn travel_with_bounds(&self, position: &Position, boundary: &Position) -> Option<Position> {
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

    pub fn travel_with_wrap(&self, position: &Position, boundary: &Position) -> Position {
        let unwrapped_position = self.travel(position);
        Position {
            row: ((unwrapped_position.row) % boundary.row + boundary.row) % boundary.row,
            column: ((unwrapped_position.column) % boundary.column + boundary.column)
                % boundary.column,
        }
    }
}

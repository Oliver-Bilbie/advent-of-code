use crate::position::Position;
use crate::traits::{Integer, One, Zero};
use std::cmp::PartialOrd;
use std::ops::{Add, Rem, Sub};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn travel<T>(&self, position: &Position<T>) -> Position<T>
    where
        T: Copy + Sub<Output = T> + Add<Output = T> + Integer + One,
    {
        match self {
            Direction::Up => Position {
                row: position.row - T::one(),
                column: position.column,
            },
            Direction::Down => Position {
                row: position.row + T::one(),
                column: position.column,
            },
            Direction::Left => Position {
                row: position.row,
                column: position.column - T::one(),
            },
            Direction::Right => Position {
                row: position.row,
                column: position.column + T::one(),
            },
        }
    }

    pub fn travel_with_bounds<T>(
        &self,
        position: &Position<T>,
        boundary: &Position<T>,
    ) -> Option<Position<T>>
    where
        T: Copy + Sub<Output = T> + Add<Output = T> + PartialOrd + Integer + One + Zero,
    {
        match self {
            Direction::Up => {
                if position.row > T::zero() {
                    Some(Position {
                        row: position.row - T::one(),
                        column: position.column,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if position.row + T::one() < boundary.row {
                    Some(Position {
                        row: position.row + T::one(),
                        column: position.column,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if position.column > T::zero() {
                    Some(Position {
                        row: position.row,
                        column: position.column - T::one(),
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if position.column + T::one() < boundary.column {
                    Some(Position {
                        row: position.row,
                        column: position.column + T::one(),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn travel_with_wrap<T>(&self, position: &Position<T>, boundary: &Position<T>) -> Position<T>
    where
        T: Copy + Sub<Output = T> + Add<Output = T> + Rem<Output = T> + Integer + One + PartialOrd,
    {
        let unwrapped_position = self.travel(position);
        Position {
            row: ((unwrapped_position.row) % boundary.row + boundary.row) % boundary.row,
            column: ((unwrapped_position.column) % boundary.column + boundary.column)
                % boundary.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_travels_up() {
        let position = Position { row: 1, column: 1 };
        let new_position = Direction::Up.travel(&position);
        let expected_position = Position { row: 0, column: 1 };
        assert_eq!(new_position, expected_position);
    }

    #[test]
    fn it_travels_right() {
        let position = Position { row: 1, column: 1 };
        let new_position = Direction::Right.travel(&position);
        let expected_position = Position { row: 1, column: 2 };
        assert_eq!(new_position, expected_position);
    }

    #[test]
    fn it_travels_down() {
        let position = Position { row: 1, column: 1 };
        let new_position = Direction::Down.travel(&position);
        let expected_position = Position { row: 2, column: 1 };
        assert_eq!(new_position, expected_position);
    }

    #[test]
    fn it_travels_left() {
        let position = Position { row: 1, column: 1 };
        let new_position = Direction::Left.travel(&position);
        let expected_position = Position { row: 1, column: 0 };
        assert_eq!(new_position, expected_position);
    }

    #[test]
    fn it_stays_in_bounds() {
        let position = Position { row: 0, column: 9 };
        let boundary = Position {
            row: 10,
            column: 10,
        };

        let under_case = Direction::Up.travel_with_bounds(&position, &boundary);
        assert!(under_case.is_none());

        let over_case = Direction::Right.travel_with_bounds(&position, &boundary);
        assert!(over_case.is_none());

        let in_bounds_case = Direction::Left.travel_with_bounds(&position, &boundary);
        assert!(in_bounds_case.is_some());
    }

    #[test]
    fn it_wraps_around() {
        let position = Position { row: 0, column: 9 };
        let boundary = Position {
            row: 10,
            column: 10,
        };

        let under_case = Direction::Up.travel_with_wrap(&position, &boundary);
        assert_eq!(under_case, Position { row: 9, column: 9 });

        let over_case = Direction::Right.travel_with_wrap(&position, &boundary);
        assert_eq!(over_case, Position { row: 0, column: 0 });
    }
}

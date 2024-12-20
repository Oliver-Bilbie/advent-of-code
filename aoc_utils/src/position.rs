use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar {}
impl Scalar for i32 {}
impl Scalar for f64 {}
impl Scalar for u16 {}
impl Scalar for usize {}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Position<T = usize> {
    pub row: T,
    pub column: T,
}

impl Position<i16> {
    pub fn from_usize(row: usize, column: usize) -> Option<Self> {
        Some(Self {
            row: i16::try_from(row).ok()?,
            column: i16::try_from(column).ok()?,
        })
    }
}

impl<T> Position<T> {
    pub fn manhattan_distance(a: &Position<T>, b: &Position<T>) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Copy + Ord,
    {
        let row_diff = if a.row > b.row {
            a.row - b.row
        } else {
            b.row - a.row
        };
        let col_diff = if a.column > b.column {
            a.column - b.column
        } else {
            b.column - a.column
        };
        row_diff + col_diff
    }
}

macro_rules! impl_ops {
    ($trait:ident, $fn:ident, $op:tt) => {
        // Position <op> Position
        impl<T: $trait<Output = T> + Copy> $trait for Position<T> {
            type Output = Position<T>;
            fn $fn(self, other: Position<T>) -> Position<T> {
                Position {
                    row: self.row $op other.row,
                    column: self.column $op other.column,
                }
            }
        }

        impl<'a, T: $trait<Output = T> + Copy> $trait<&'a Position<T>> for Position<T> {
            type Output = Position<T>;
            fn $fn(self, other: &'a Position<T>) -> Position<T> {
                Position {
                    row: self.row $op other.row,
                    column: self.column $op other.column,
                }
            }
        }

        impl<'b, T: $trait<Output = T> + Copy> $trait<Position<T>> for &'b Position<T> {
            type Output = Position<T>;
            fn $fn(self, other: Position<T>) -> Position<T> {
                Position {
                    row: self.row $op other.row,
                    column: self.column $op other.column,
                }
            }
        }

        impl<'a, 'b, T: $trait<Output = T> + Copy> $trait<&'b Position<T>> for &'a Position<T> {
            type Output = Position<T>;
            fn $fn(self, other: &'b Position<T>) -> Position<T> {
                Position {
                    row: self.row $op other.row,
                    column: self.column $op other.column,
                }
            }
        }

        // Position <op> scalar
        impl<T, U> $trait<U> for Position<T>
        where
            T: $trait<Output = T> + Copy,
            U: Scalar + Copy + Into<T>,
        {
            type Output = Position<T>;
            fn $fn(self, scalar: U) -> Position<T> {
                let scalar = scalar.into();
                Position {
                    row: self.row $op scalar,
                    column: self.column $op scalar,
                }
            }
        }

        impl<'a, T, U> $trait<U> for &'a Position<T>
        where
            T: $trait<Output = T> + Copy,
            U: Scalar + Copy + Into<T>,
        {
            type Output = Position<T>;
            fn $fn(self, scalar: U) -> Position<T> {
                let scalar = scalar.into();
                Position {
                    row: self.row $op scalar,
                    column: self.column $op scalar,
                }
            }
        }
    };
}

impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_from_usize() {
        let row: usize = 1;
        let column: usize = 2;
        let pos = Position::from_usize(row, column).unwrap();
        assert_eq!(pos.row, 1 as i16);
        assert_eq!(pos.column, 2 as i16);
    }

    #[test]
    fn it_adds_positions() {
        let pos_1 = Position { row: 1, column: 2 };
        let pos_2 = Position { row: 3, column: 4 };
        let result = pos_1 + pos_2;
        assert_eq!(result.row, 4);
        assert_eq!(result.column, 6);
    }

    #[test]
    fn it_subtracts_positions() {
        let pos_1 = Position { row: 1, column: 2 };
        let pos_2 = Position { row: 3, column: 4 };
        let result = pos_2 - pos_1;
        assert_eq!(result.row, 2);
        assert_eq!(result.column, 2);
    }

    #[test]
    fn it_multiplies_positions() {
        let pos_1 = Position { row: 1, column: 2 };
        let pos_2 = Position { row: 3, column: 4 };
        let result = pos_1 * pos_2;
        assert_eq!(result.row, 3);
        assert_eq!(result.column, 8);
    }

    #[test]
    fn it_divides_positions() {
        let pos_1 = Position::<f32> {
            row: 2.0,
            column: 3.0,
        };
        let pos_2 = Position::<f32> {
            row: 3.0,
            column: 9.0,
        };
        let result = pos_2 / pos_1;
        assert_eq!(result.row, 1.5);
        assert_eq!(result.column, 3.0);
    }

    #[test]
    fn it_adds_scalars() {
        let pos = Position { row: 1, column: 2 };
        let result = pos + 1;
        assert_eq!(result.row, 2);
        assert_eq!(result.column, 3);
    }

    #[test]
    fn it_subtracts_scalars() {
        let pos = Position { row: 1, column: 2 };
        let result = pos - 1;
        assert_eq!(result.row, 0);
        assert_eq!(result.column, 1);
    }

    #[test]
    fn it_multiplies_scalars() {
        let pos = Position { row: 1, column: 2 };
        let result = pos * 3;
        assert_eq!(result.row, 3);
        assert_eq!(result.column, 6);
    }

    #[test]
    fn it_divides_scalars() {
        let pos = Position::<f64> {
            row: 1.0,
            column: 2.0,
        };
        let result = pos / 2.0;
        assert_eq!(result.row, 0.5);
        assert_eq!(result.column, 1.0);
    }
}

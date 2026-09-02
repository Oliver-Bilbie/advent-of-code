use crate::position::Position;

pub struct Grid<T> {
    data: Vec<T>,
    size: Position,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let row = data.len();
        let column = match data.first() {
            Some(r) => r.len(),
            None => 0,
        };
        Grid {
            data: data.into_iter().flatten().collect(),
            size: Position { row, column },
        }
    }

    pub fn from_str(input: &str, mapper: fn(&str) -> Vec<T>) -> Self {
        let row = input.lines().count();
        let column = match input.lines().next() {
            Some(r) => r.len(),
            None => 0,
        };
        let data = input.lines().flat_map(|line| mapper(line)).collect();
        Grid {
            data,
            size: Position { row, column },
        }
    }

    pub fn get(&self, position: &Position<usize>) -> Option<&T> {
        self.data
            .get(self.size.column * position.row + position.column)
    }

    pub fn get_mut(&mut self, position: &Position<usize>) -> Option<&mut T> {
        self.data
            .get_mut(self.size.column * position.row + position.column)
    }

    pub fn set(&mut self, position: &Position<usize>, value: T) {
        if let Some(v) = self.get_mut(position) {
            *v = value;
        }
    }

    pub fn apply(&mut self, operation: fn(&T) -> T) {
        self.data.iter_mut().for_each(|v| *v = operation(v));
    }

    pub fn position(&self, predicate: fn(&T) -> bool) -> Option<Position> {
        match self.data.iter().position(predicate) {
            Some(i) => Some(Position {
                row: i / self.size.column,
                column: i % self.size.column,
            }),
            None => None,
        }
    }

    pub fn size(&self) -> &Position {
        &self.size
    }

    pub fn positions(&self) -> Vec<Position> {
        (0..self.size.row)
            .flat_map(|row| (0..self.size.column).map(move |column| Position { row, column }))
            .collect()
    }

    pub fn print(&self, mapper: fn(&T) -> String) {
        for row in 0..self.size.row {
            for column in 0..self.size.column {
                print!("{}", mapper(self.get(&Position { row, column }).unwrap()));
            }
            println!();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new_and_dimensions() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let grid = Grid::new(input);

        assert_eq!(*grid.size(), Position { row: 3, column: 3 });
        assert_eq!(grid.data, expected);
    }

    #[test]
    fn test_get_and_set() {
        let mut grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        // Test getting values
        assert_eq!(grid.get(&Position { row: 0, column: 0 }), Some(&1));
        assert_eq!(grid.get(&Position { row: 2, column: 2 }), Some(&9));
        assert_eq!(grid.get(&Position { row: 3, column: 3 }), None);

        // Test setting values
        grid.set(&Position { row: 1, column: 1 }, 42);
        assert_eq!(grid.get(&Position { row: 1, column: 1 }), Some(&42));
    }

    #[test]
    fn test_from_str() {
        let input = "\
123
456
789";
        let grid = Grid::from_str(input, |s| {
            s.chars().map(|c| c.to_digit(10).unwrap()).collect()
        });

        assert_eq!(*grid.size(), Position { row: 3, column: 3 });
        assert_eq!(grid.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_set_out_of_bounds() {
        let mut grid = Grid::new(vec![vec![1, 2], vec![3, 4]]);

        // Attempt to set a value outside of the grid's bounds
        grid.set(
            &Position {
                row: 10,
                column: 10,
            },
            42,
        );
        // Grid should remain unchanged
        assert_eq!(grid.data, vec![1, 2, 3, 4]);
    }
}

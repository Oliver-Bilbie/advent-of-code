use crate::position::Position;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    pub fn from_str<F>(input: &str, mut mapper: F) -> Self
    where
        F: FnMut(char) -> T,
    {
        let data = input
            .lines()
            .map(|line| line.chars().map(&mut mapper).collect())
            .collect();
        Grid { data }
    }

    pub fn get(&self, position: &Position<usize>) -> Option<&T> {
        self.data
            .get(position.row)
            .and_then(|row| row.get(position.column))
    }

    pub fn get_mut(&mut self, position: &Position<usize>) -> Option<&mut T> {
        self.data
            .get_mut(position.row)
            .and_then(|row| row.get_mut(position.column))
    }

    pub fn set(&mut self, position: &Position<usize>, value: T) {
        if let Some(row) = self.data.get_mut(position.row) {
            if let Some(cell) = row.get_mut(position.column) {
                *cell = value;
            }
        }
    }

    pub fn dimensions(&self) -> Position {
        let row = self.data.len();
        let column = self.data.first().map_or(0, |row| row.len());
        Position { row, column }
    }

    pub fn print<F>(&self, mut mapper: F)
    where
        F: FnMut(&T) -> char,
    {
        self.data.iter().for_each(|row| {
            println!("{}", row.iter().map(&mut mapper).collect::<String>());
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new_and_dimensions() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(data.clone());

        assert_eq!(grid.dimensions(), Position { row: 3, column: 3 });
        assert_eq!(grid.data, data);
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
        let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap());

        assert_eq!(grid.dimensions(), Position { row: 3, column: 3 });
        assert_eq!(
            grid.data,
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]
        );
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
        assert_eq!(grid.data, vec![vec![1, 2], vec![3, 4],]);
    }
}

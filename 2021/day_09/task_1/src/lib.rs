use aoc_utils::{direction::ALL_DIRECTIONS, grid::Grid, position::Position};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The sum of the risk levels is: {}", result(input));
}

fn result(input: &str) -> u64 {
    let mut total = 0;
    let grid = Grid::<u8>::from_str(input, |s| {
        s.chars()
            .map(|c| c.to_digit(10).expect("value should be numeric") as u8)
            .collect()
    });

    for row in 0..grid.size().row {
        for column in 0..grid.size().column {
            let pos = Position { row, column };
            let height = grid.get(&pos).unwrap();
            if ALL_DIRECTIONS
                .iter()
                .all(|dir| match dir.travel_with_bounds(&pos, &grid.size()) {
                    Some(adj) => {
                        let adj_height = grid.get(&adj).unwrap();
                        return adj_height > height;
                    }
                    None => true,
                })
            {
                total += *height as u64 + 1;
            };
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 15);
    }
}

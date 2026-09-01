use aoc_utils::{direction::ALL_DIRECTIONS, grid::Grid, position::Position};
use std::collections::{BinaryHeap, VecDeque};
use wasm_bindgen::prelude::*;

const MAX_HEIGHT: u8 = 9;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "The product of the sizes of the three largest basins is: {}",
        result(input)
    );
}

fn result(input: &str) -> u64 {
    let mut grid = Grid::<u8>::from_str(input, |s| {
        s.chars()
            .map(|c| c.to_digit(10).expect("value should be numeric") as u8)
            .collect()
    });
    let minima = find_minima(&grid);
    let mut basin_sizes: BinaryHeap<u8> = minima
        .iter()
        .map(|p| fill_basin(&mut grid, p.clone()))
        .collect();

    let mut result = 1;
    for _ in 0..3 {
        result *= basin_sizes.pop().expect("not enough basins") as u64
    }
    result
}

fn find_minima(grid: &Grid<u8>) -> Vec<Position> {
    let mut minima = Vec::<Position>::new();
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
                minima.push(pos);
            };
        }
    }
    minima
}

// Flood-fill the basin and return its size
fn fill_basin(grid: &mut Grid<u8>, start: Position) -> u8 {
    let mut size = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(p) = queue.pop_front() {
        let h = grid.get_mut(&p).unwrap();
        if *h == MAX_HEIGHT {
            continue;
        }
        *h = MAX_HEIGHT;
        size += 1;

        for d in ALL_DIRECTIONS {
            if let Some(next) = d.travel_with_bounds(&p, grid.size()) {
                queue.push_back(next.clone());
            }
        }
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 1134);
    }
}

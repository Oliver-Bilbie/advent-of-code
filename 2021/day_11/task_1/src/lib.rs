use aoc_utils::{direction::Direction, grid::Grid};
use wasm_bindgen::prelude::*;

fn can_flash(energy: &Option<u8>) -> bool {
    match energy {
        Some(v) => *v > 9,
        None => false,
    }
}

fn simulate_step(grid: &mut Grid<Option<u8>>) -> u64 {
    let mut flash_count = 0;
    // increment all energy levels
    grid.apply(|x| Some(x.unwrap_or(0) + 1));

    while let Some(flash_pos) = grid.position(|e| can_flash(e)) {
        // set energy as 'none' once an octopus has flashed
        flash_count += 1;
        grid.set(&flash_pos, None);
        for dirs in [
            [Direction::Up, Direction::Left].as_slice(),
            [Direction::Up].as_slice(),
            [Direction::Up, Direction::Right].as_slice(),
            [Direction::Left].as_slice(),
            [Direction::Right].as_slice(),
            [Direction::Down, Direction::Left].as_slice(),
            [Direction::Down].as_slice(),
            [Direction::Down, Direction::Right].as_slice(),
        ] {
            let mut neighbor = Some(flash_pos.clone());
            for d in dirs {
                if neighbor.is_none() {
                    break;
                }
                neighbor = d.travel_with_bounds(&neighbor.unwrap(), grid.size())
            }
            if let Some(pos) = neighbor
                && let Some(energy) = grid.get_mut(&pos).unwrap()
            {
                *energy += 1;
            }
        }
    }

    // reset energy levels after flashes
    grid.apply(|x| match x {
        Some(v) => Some(*v),
        None => Some(0),
    });
    flash_count
}

fn result(input: &str, total_steps: u8) -> u64 {
    let mut grid = Grid::from_str(input, |line| {
        line.chars()
            .map(|c| Some(c.to_digit(10).expect("character is not a digit") as u8))
            .collect()
    });
    (0..total_steps).map(|_| simulate_step(&mut grid)).sum()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("There are {} flashes after 100 steps", result(input, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input, 100), 1656);
    }
}

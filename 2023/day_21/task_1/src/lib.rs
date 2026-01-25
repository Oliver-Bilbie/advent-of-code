use aoc_utils::direction::Direction;
use aoc_utils::position::Position;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

struct Tiles {
    is_accessible: Vec<bool>,
    width: usize,
}
impl Tiles {
    fn new(height: usize, width: usize) -> Tiles {
        Tiles {
            is_accessible: vec![false; width * height],
            width,
        }
    }

    fn set(&mut self, row: usize, column: usize, value: bool) {
        self.is_accessible[row * self.width + column] = value;
    }

    fn get(&self, position: &Position<i32>) -> bool {
        self.is_accessible[position.row as usize * self.width + position.column as usize]
    }
}

fn read_tiles(input: &str) -> (Tiles, Position<i32>) {
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().len();
    let mut tiles = Tiles::new(height, width);
    let mut start_position: Option<Position<i32>> = None;

    for (row, line) in input.lines().enumerate() {
        for (column, item) in line.chars().enumerate() {
            match item {
                '.' => tiles.set(row, column, true),
                'S' => {
                    assert!(start_position.is_none());
                    start_position = Some(Position {
                        row: row as i32,
                        column: column as i32,
                    });
                    tiles.set(row, column, true);
                }
                '#' => tiles.set(row, column, false),
                _ => panic!("invalid character in input: {}", item),
            };
        }
    }

    return (
        tiles,
        start_position.expect("the input to contain a start position"),
    );
}

fn result(input: &str, steps: u32) -> u64 {
    let (tiles, start_pos) = read_tiles(input);
    let mut heads = HashSet::<Position<i32>>::new();
    heads.insert(start_pos);
    let mut next_heads = HashSet::<Position<i32>>::new();

    for _ in 0..steps {
        for pos in &heads {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next_pos = direction.travel(&pos);
                if tiles.get(&next_pos) {
                    next_heads.insert(next_pos);
                }
            }
        }

        std::mem::swap(&mut heads, &mut next_heads);
        next_heads.clear();
    }

    return heads.len() as u64;
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "The elf can reach {} garden plots in exactly 64 steps",
        result(input, 64)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input, 6), 16);
    }
}

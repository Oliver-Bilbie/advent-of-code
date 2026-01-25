use aoc_utils::direction::Direction;
use aoc_utils::position::Position;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

struct Tiles {
    is_accessible: Vec<bool>,
    width: usize,
    height: usize,
}
impl Tiles {
    fn new(height: usize, width: usize) -> Tiles {
        Tiles {
            is_accessible: vec![false; width * height],
            width,
            height,
        }
    }

    fn set(&mut self, row: usize, column: usize, value: bool) {
        self.is_accessible[row * self.width + column] = value;
    }

    fn get_with_wrap(&self, position: &Position<i32>) -> bool {
        let wrapped_row = position.row.rem_euclid(self.height as i32) as usize;
        let wrapped_column = position.column.rem_euclid(self.width as i32) as usize;
        self.is_accessible[wrapped_row * self.width + wrapped_column]
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

fn result(input: &str) -> u64 {
    let required_steps = 26501365;

    let (tiles, start_pos) = read_tiles(input);
    let mut heads = HashSet::<Position<i32>>::new();
    heads.insert(start_pos);
    let mut next_heads = HashSet::<Position<i32>>::new();

    assert!(tiles.width == tiles.height, "the garden is not square");

    // We are going to build a quadratic equation in terms of the number of complete gardens
    // (as opposed to individual tiles). The offset here represents the number of tiles we have
    // remaining to reach the destination, since the goal is not an integer number of plots away
    // from the starting position.
    let offset = required_steps % tiles.width;
    let mut samples: Vec<i64> = Vec::new();

    for i in 1..=offset + 2 * tiles.width {
        for pos in &heads {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next_pos = direction.travel(&pos);
                if tiles.get_with_wrap(&next_pos) {
                    next_heads.insert(next_pos);
                }
            }
        }

        std::mem::swap(&mut heads, &mut next_heads);
        next_heads.clear();

        if i >= offset && (i - offset) % tiles.width == 0 {
            samples.push(heads.len() as i64);
            if samples.len() == 3 {
                break;
            }
        }
    }

    // Due to how the input is designed, we can fit a quadratic and use that to find our solution.
    // This does not generalize to arbitrary inputs, but such is AoC.
    let d1_0 = samples[1] - samples[0];
    let d1_1 = samples[2] - samples[1];
    let d2 = d1_1 - d1_0;

    let a = d2 / 2;
    let b = d1_0 - a;
    let c = samples[0];

    let k = ((required_steps - offset) / tiles.width) as i64;
    return (a * k * k + b * k + c) as u64;
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "The elf can reach {} garden plots in exactly 26501365 steps",
        result(input)
    );
}

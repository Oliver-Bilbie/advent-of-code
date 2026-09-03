use aoc_utils::position::Position;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

struct Fold {
    is_horizontal: bool,
    position: usize,
}

fn read_dots(input: &str) -> HashSet<Position> {
    input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Position {
                row: y.parse::<usize>().unwrap(),
                column: x.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn read_folds(input: &str) -> Vec<Fold> {
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let (l, r) = &l[11..].split_once('=').unwrap();
            let is_horizontal = *l == "y";
            let position = r.parse::<usize>().unwrap();
            Fold {
                is_horizontal,
                position,
            }
        })
        .collect()
}

fn result(input: &str) -> u64 {
    let mut dots = read_dots(input);
    let folds = read_folds(input);
    let f = folds.first().unwrap();

    dots = dots
        .iter()
        .map(|pos| {
            if f.is_horizontal && pos.row > f.position {
                return Position {
                    row: 2 * f.position - pos.row,
                    column: pos.column,
                };
            }
            if !f.is_horizontal && pos.column > f.position {
                return Position {
                    row: pos.row,
                    column: 2 * f.position - pos.column,
                };
            }
            pos.clone()
        })
        .collect();

    return dots.len() as u64;
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("{} dots are visible", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 17);
    }
}

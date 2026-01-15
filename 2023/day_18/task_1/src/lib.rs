use aoc_utils::{direction::Direction, position::Position};
use std::collections::{HashSet, VecDeque};
use wasm_bindgen::prelude::*;

struct Instruction {
    direction: Direction,
    distance: u8,
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut items = line.split_whitespace();
            let (dir_str, dist_str) = (items.next().unwrap(), items.next().unwrap());
            let direction = match dir_str {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("{} is not a valid direction", dir_str),
            };
            let distance = dist_str.parse::<u8>().unwrap();
            Instruction {
                direction,
                distance,
            }
        })
        .collect()
}

fn dig_trench(occupied: &mut HashSet<Position>, instructions: &Vec<Instruction>) {
    let mut pos = Position { row: 0, column: 0 };
    occupied.insert(pos.clone());

    for i in instructions {
        for _ in 0..i.distance {
            pos = i.direction.travel(&pos);
            occupied.insert(pos.clone());
        }
    }
}

fn flood_fill(start: Position, occupied: &mut HashSet<Position>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(position) = queue.pop_front() {
        if !occupied.insert(position.clone()) {
            continue;
        }

        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            queue.push_back(direction.travel(&position));
        }
    }
}

fn result(input: &str) -> u64 {
    let instructions = read_instructions(input);
    let mut occupied = HashSet::<Position>::new();

    dig_trench(&mut occupied, &instructions);
    flood_fill(Position { row: 1, column: 1 }, &mut occupied);

    return occupied.len() as u64;
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "The lagoon could hold {} cubic meters of lava",
        result(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 62);
    }
}

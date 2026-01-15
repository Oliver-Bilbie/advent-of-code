use aoc_utils::{direction::Direction, position::Position};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

struct Range {
    begin: i32,
    end: i32,
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let hex_str = line.split_whitespace().last().unwrap();
            let direction = match hex_str.chars().nth_back(1).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!(
                    "{} is not a valid direction",
                    hex_str.chars().last().unwrap()
                ),
            };
            let distance = i32::from_str_radix(hex_str.get(2..7).unwrap(), 16).unwrap();
            Instruction {
                direction,
                distance,
            }
        })
        .collect()
}

fn dig_trench(instructions: &Vec<Instruction>) -> HashMap<i32, Vec<i32>> {
    let mut trench = HashMap::<i32, Vec<i32>>::new();
    let mut pos = Position { row: 0, column: 0 };
    trench
        .entry(pos.row)
        .or_insert_with(Vec::new)
        .push(pos.column);

    for i in instructions {
        for _ in 0..i.distance {
            pos = i.direction.travel(&pos);
            trench
                .entry(pos.row)
                .or_insert_with(Vec::new)
                .push(pos.column);
        }
    }

    return trench;
}

fn find_ranges(columns: &Vec<i32>) -> Vec<Range> {
    let mut ranges: Vec<Range> = vec![];
    let mut begin = columns.first().unwrap().clone();
    let mut end = begin;

    for col in columns.iter().skip(1) {
        if *col == end + 1 {
            end += 1;
        } else {
            ranges.push(Range { begin, end });
            begin = col.clone();
            end = begin;
        }
    }
    ranges.push(Range { begin, end });

    return ranges;
}

fn check_if_boundary(row: i32, range: &Range, trench: &HashMap<i32, Vec<i32>>) -> bool {
    // if the trench has a width of 1, it must be a boundary
    if range.begin == range.end {
        return true;
    }

    // if the trench comes into and out of the row from the same direction, it is not a boundary
    for offset in [1, -1] {
        match trench.get(&(row + offset)) {
            Some(next_row) => {
                if next_row.contains(&range.begin) && next_row.contains(&range.end) {
                    return false;
                }
            }
            None => {}
        }
    }

    return true;
}

fn calculate_area(trench: &HashMap<i32, Vec<i32>>) -> u64 {
    let mut total_area: u64 = 0;

    for (row, columns) in trench {
        let mut is_inside = false;

        let ranges = find_ranges(columns);
        for (i, range) in ranges.iter().enumerate() {
            // count trench tiles
            total_area += (1 + range.end - range.begin) as u64;

            // if we are inside of the lagoon, add the tiles between the ranges
            if is_inside {
                total_area += (range.begin - ranges[i - 1].end - 1) as u64;
            }

            // evaluate whether we are now inside or outside of the lagoon
            if check_if_boundary(*row, range, trench) {
                is_inside = !is_inside;
            }
        }
    }

    return total_area;
}

fn result(input: &str) -> u64 {
    let instructions = read_instructions(input);
    let trench = dig_trench(&instructions);
    return calculate_area(&trench);
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
        assert_eq!(result(&input), 952408144115);
    }

    #[test]
    fn it_reads_the_instructions() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_instructions = read_instructions(&input);
        let expected_instructions = vec![
            Instruction {
                direction: Direction::Right,
                distance: 461937,
            },
            Instruction {
                direction: Direction::Down,
                distance: 56407,
            },
            Instruction {
                direction: Direction::Right,
                distance: 356671,
            },
            Instruction {
                direction: Direction::Down,
                distance: 863240,
            },
            Instruction {
                direction: Direction::Right,
                distance: 367720,
            },
        ];

        actual_instructions
            .iter()
            .zip(expected_instructions)
            .for_each(|(actual, expected)| {
                assert_eq!(*actual, expected);
            });
    }
}

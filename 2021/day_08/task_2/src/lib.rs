pub mod segments;
use crate::segments::*;
use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct Digit {
    _items: [Option<char>; 7],
}

impl Digit {
    fn new() -> Digit {
        Digit { _items: [None; 7] }
    }

    fn get(&self, segment: &Segment) -> &Option<char> {
        self._items.get(segment.as_usize()).unwrap()
    }

    fn insert(&mut self, segment: &Segment, name: char) {
        self._items[segment.as_usize()] = Some(name);
    }

    fn try_merge(&mut self, other: Digit) -> bool {
        for i in 0..7 {
            if let Some(current) = self._items[i]
                && let Some(new) = other._items[i]
            {
                if current != new {
                    return false;
                }
            }
        }
        for i in 0..7 {
            if other._items[i].is_some() {
                self._items[i] = other._items[i];
            }
        }
        true
    }

    fn decode(&self, names: &str) -> Option<usize> {
        let segments: Vec<Segment> = [
            Segment::Top,
            Segment::TopLeft,
            Segment::TopRight,
            Segment::Middle,
            Segment::BottomLeft,
            Segment::BottomRight,
            Segment::Bottom,
        ]
        .iter()
        .filter(|seg| names.contains(self.get(seg).expect("name does not appear in digit")))
        .cloned()
        .collect();

        [
            SEGMENTS0.as_slice(),
            SEGMENTS1.as_slice(),
            SEGMENTS2.as_slice(),
            SEGMENTS3.as_slice(),
            SEGMENTS4.as_slice(),
            SEGMENTS5.as_slice(),
            SEGMENTS6.as_slice(),
            SEGMENTS7.as_slice(),
            SEGMENTS8.as_slice(),
            SEGMENTS9.as_slice(),
        ]
        .iter()
        .position(|&sig| sig == segments)
    }
}

fn possible_segments(signal: &str) -> Vec<&[Segment]> {
    match signal.len() {
        2 => vec![&SEGMENTS1],
        3 => vec![&SEGMENTS7],
        4 => vec![&SEGMENTS4],
        5 => vec![&SEGMENTS2, &SEGMENTS3, &SEGMENTS5],
        6 => vec![&SEGMENTS0, &SEGMENTS6, &SEGMENTS9],
        7 => vec![&SEGMENTS8],
        _ => panic!("invalid sement length"),
    }
}

fn backtrack(digit: Digit, inputs: &Vec<&str>, n: usize) -> Option<Digit> {
    let display = match inputs.get(n) {
        Some(v) => v,
        None => {
            return Some(digit);
        }
    };
    let possible = possible_segments(display);
    for num in possible {
        // try each mapping name->segment
        for perm in display.chars().permutations(display.len()) {
            let mut mapping = Digit::new();
            for (i, seg) in num.iter().enumerate() {
                mapping.insert(seg, perm[i]);
            }
            if !mapping.try_merge(digit.clone()) {
                continue;
            }
            let result = backtrack(mapping, inputs, n + 1);
            if result.is_some() {
                return result;
            }
        }
    }
    None
}

fn find_mapping(inputs: &Vec<&str>) -> Digit {
    match backtrack(Digit::new(), inputs, 0) {
        Some(v) => v,
        None => panic!("no solution was found"),
    }
}

fn decode_output(outputs: &Vec<&str>, mapping: Digit) -> u64 {
    let mut result = 0;
    for names in outputs {
        result = 10 * result + mapping.decode(names).expect("invalid digit") as u64;
    }
    result
}

fn result(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (inputs, outputs) = line
                .split_once('|')
                .expect("input should contain a '|' delimiter");
            let mut inputs: Vec<&str> = inputs.split_whitespace().collect();
            // since input order doesn't matter, we can sort inputs by length to reduce the backtracking
            // solution space
            inputs.sort_by(|a, b| a.len().cmp(&b.len()));
            let outputs: Vec<&str> = outputs.split_whitespace().collect();
            let mapping = find_mapping(&inputs);
            decode_output(&outputs, mapping)
        })
        .sum()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The sum of the output values is {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_the_example_mapping() {
        let inputs = vec![
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        let mapping = find_mapping(&inputs);
        assert_eq!(*mapping.get(&Segment::Top), Some('d'));
        assert_eq!(*mapping.get(&Segment::TopLeft), Some('e'));
        assert_eq!(*mapping.get(&Segment::TopRight), Some('a'));
        assert_eq!(*mapping.get(&Segment::Middle), Some('f'));
        assert_eq!(*mapping.get(&Segment::BottomLeft), Some('g'));
        assert_eq!(*mapping.get(&Segment::BottomRight), Some('b'));
        assert_eq!(*mapping.get(&Segment::Bottom), Some('c'));
    }

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 61229);
    }
}

use crate::helpers::keypad::Keypad;
use std::collections::HashMap;

pub mod helpers;

pub fn solve(input: &str) -> String {
    let result = input
        .lines()
        .fold(0, |acc, x| acc + input_len(x, 4) * numeric_part(x));

    return format!("Sum of complexities: {}", result);
}

fn numeric_part(code: &str) -> u64 {
    let mut value = 0;
    let mut mult = 1;

    for c in code.chars().rev() {
        if let Some(v) = c.to_digit(10) {
            value += v * mult;
            mult *= 10;
        }
    }

    return value as u64;
}

fn input_len(code: &str, keypad_count: u8) -> u64 {
    if keypad_count < 1 {
        panic!("the number of keypads must be greater than zero");
    }

    if keypad_count == 1 {
        return code.len() as u64;
    }

    // Storing the full path as a string becomes impractical once we get
    // a few keypads deep. Therefore we will store all of the keypairs
    // that make it up, along with the number of them present in the path.
    let mut path_pairs = HashMap::<(char, char), u64>::new();

    for i in 0..code.len() {
        let start = match i == 0 {
            true => 'A',
            false => code.chars().nth(i - 1).unwrap(),
        };
        let end = code.chars().nth(i).unwrap();

        let path = Keypad::NUMERIC.path(start, end);

        for j in 0..path.len() {
            let s = match j == 0 {
                true => 'A',
                false => path.chars().nth(j - 1).unwrap(),
            };
            let e = path.chars().nth(j).unwrap();

            let pair_count = path_pairs.entry((s, e)).or_insert(0);
            *pair_count += 1;
        }
    }

    let mut next_path_pairs = HashMap::<(char, char), u64>::new();

    for _ in 2..keypad_count {
        for ((start, end), count) in &path_pairs {
            let path = Keypad::DIRECTIONAL.path(start.clone(), end.clone());

            for j in 0..path.len() {
                let s = match j == 0 {
                    true => 'A',
                    false => path.chars().nth(j - 1).unwrap(),
                };
                let e = path.chars().nth(j).unwrap();

                let pair_count = next_path_pairs.entry((s, e)).or_insert(0);
                *pair_count += count;
            }
        }

        std::mem::swap(&mut path_pairs, &mut next_path_pairs);
        next_path_pairs.clear();
    }

    return path_pairs.iter().fold(0, |acc, (_, v)| acc + v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_length_example_1() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(0).unwrap();

        assert_eq!(input_len(code, 4), 68);
    }

    #[test]
    fn it_finds_length_example_2() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(1).unwrap();

        assert_eq!(input_len(code, 4), 60);
    }

    #[test]
    fn it_finds_length_example_3() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(2).unwrap();

        assert_eq!(input_len(code, 4), 68);
    }

    #[test]
    fn it_finds_length_example_4() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(3).unwrap();

        assert_eq!(input_len(code, 4), 64);
    }

    #[test]
    fn it_finds_length_example_5() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(4).unwrap();

        assert_eq!(input_len(code, 4), 64);
    }

    #[test]
    fn it_finds_numeric_part_example_1() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(0).unwrap();

        assert_eq!(numeric_part(code), 29);
    }

    #[test]
    fn it_finds_numeric_part_example_2() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(1).unwrap();

        assert_eq!(numeric_part(code), 980);
    }

    #[test]
    fn it_finds_numeric_part_example_3() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(2).unwrap();

        assert_eq!(numeric_part(code), 179);
    }

    #[test]
    fn it_finds_numeric_part_example_4() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(3).unwrap();

        assert_eq!(numeric_part(code), 456);
    }

    #[test]
    fn it_finds_numeric_part_example_5() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let code = input.lines().nth(4).unwrap();

        assert_eq!(numeric_part(code), 379);
    }
}

use wasm_bindgen::prelude::*;

struct Profiles {
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("There are {} unique combinations", result(input));
}

fn result(input: &str) -> usize {
    let mut combinations = 0;
    let profiles = read_profiles(input);
    let locks = profiles.locks;
    let keys = profiles.keys;

    const MAX_HEIGHT: u8 = 5;

    for lock in &locks {
        for key in &keys {
            let sum = lock.iter().zip(key).map(|(a, b)| a + b);
            if sum.filter(|v| *v > MAX_HEIGHT).count() == 0 {
                combinations += 1;
            }
        }
    }

    combinations
}

fn read_profiles(input: &str) -> Profiles {
    const LOCK_HEIGHT: usize = 7;

    let mut locks = Vec::<[u8; 5]>::new();
    let mut keys = Vec::<[u8; 5]>::new();

    let lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for object in lines.chunks(LOCK_HEIGHT) {
        let top_left = object.first().unwrap().first().unwrap();
        match top_left {
            '#' => {
                locks.push(read_lock(object));
            }
            '.' => {
                keys.push(read_key(object));
            }
            _ => panic!("the input contains an invalid character"),
        }
    }

    Profiles { locks, keys }
}

fn read_lock(schematic: &[Vec<char>]) -> [u8; 5] {
    let mut heights = [0; 5];

    for depth in 1..schematic.len() {
        for pin in 0..5 {
            let value = schematic.get(depth).unwrap().get(pin).unwrap();
            if *value == '#' {
                heights[pin] = depth as u8;
            }
        }
    }

    heights
}

fn read_key(schematic: &[Vec<char>]) -> [u8; 5] {
    let mut heights = [0; 5];

    for depth in 1..schematic.len() {
        for pin in 0..5 {
            let value = schematic
                .get(schematic.len() - depth - 1)
                .unwrap()
                .get(pin)
                .unwrap();
            if *value == '#' {
                heights[pin] = depth as u8;
            }
        }
    }

    heights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_profiles() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let profiles = read_profiles(&input);

        let expected_locks = vec![[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]];
        let expected_keys = vec![[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1]];

        assert_eq!(profiles.locks, expected_locks);
        assert_eq!(profiles.keys, expected_keys);
    }

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 3);
    }
}

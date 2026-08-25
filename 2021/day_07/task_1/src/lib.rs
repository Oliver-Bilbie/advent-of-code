use wasm_bindgen::prelude::*;

struct Crabs {
    positions: Vec<u16>,
}

impl Crabs {
    fn from_input(input: &str) -> Crabs {
        let mut positions: Vec<u16> = input
            .trim()
            .split(',')
            .map(|p| p.parse().expect("crab position should be an integer"))
            .collect();
        positions.sort();
        Crabs { positions }
    }

    fn total(&self) -> usize {
        self.positions.len()
    }

    fn count_smallest(&self) -> usize {
        let value = self
            .positions
            .first()
            .expect("positions should not be empty")
            .clone();

        for (count, pos) in self.positions.iter().enumerate() {
            if *pos > value {
                return count;
            }
        }
        self.positions.len()
    }

    fn count_largest(&self) -> usize {
        let value = self
            .positions
            .last()
            .expect("positions should not be empty")
            .clone();

        for (count, pos) in self.positions.iter().rev().enumerate() {
            if *pos < value {
                return count;
            }
        }
        self.positions.len()
    }

    fn increment_smallest(&mut self, count: usize) {
        for i in 0..count {
            self.positions[i] += 1;
        }
    }

    fn decrement_largest(&mut self, count: usize) {
        let n = self.positions.len() - 1;
        for i in 0..count {
            self.positions[n - i] -= 1;
        }
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("{} units of fuel are required", result(input));
}

fn result(input: &str) -> u64 {
    let mut fuel_used = 0;
    let mut crabs = Crabs::from_input(input);

    loop {
        let sml = crabs.count_smallest();
        let lrg = crabs.count_largest();

        if sml == crabs.total() {
            return fuel_used;
        }

        if sml <= lrg {
            fuel_used += sml as u64;
            crabs.increment_smallest(sml);
        } else {
            fuel_used += lrg as u64;
            crabs.decrement_largest(lrg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 37);
    }
}

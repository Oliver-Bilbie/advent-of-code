use wasm_bindgen::prelude::*;

struct UpdateState {
    size: usize,
    cost: u64,
}

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

    let mut increase_state = UpdateState { size: 0, cost: 0 };
    let mut decrease_state = UpdateState { size: 0, cost: 0 };

    loop {
        let sml = crabs.count_smallest();
        let lrg = crabs.count_largest();

        if sml == crabs.total() {
            return fuel_used;
        }

        let increase_cost =
            increase_state.cost + increase_state.size as u64 + (sml - increase_state.size) as u64;
        let decrease_cost =
            decrease_state.cost + decrease_state.size as u64 + (lrg - decrease_state.size) as u64;

        if increase_cost <= decrease_cost {
            fuel_used += increase_cost;
            increase_state.size = sml;
            increase_state.cost = increase_cost;
            crabs.increment_smallest(sml);
        } else {
            fuel_used += decrease_cost;
            decrease_state.size = lrg;
            decrease_state.cost = decrease_cost;
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
        assert_eq!(result(&input), 168);
    }
}

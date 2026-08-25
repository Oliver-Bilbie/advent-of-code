use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("After 256 days there are {} lanternfish", result(input));
}

fn result(input: &str) -> u64 {
    let mut counts: [u64; 9] = [0; 9]; // timer -> count
    for t_str in input.trim().split(',') {
        let t: usize = t_str.parse().expect("timer should be an integer");
        counts[t] += 1;
    }

    for _ in 0..256 {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    return counts.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 26984457539);
    }
}

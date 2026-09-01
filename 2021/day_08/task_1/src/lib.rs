use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The values appear {} times", result(input));
}

fn result(input: &str) -> u64 {
    const UNIQUE_SEGMENT_COUNTS: [usize; 4] = [2, 4, 3, 7];
    input
        .lines()
        .flat_map(|l| {
            l.split_once('|')
                .expect("line should contain a '|' delimiter")
                .1
                .split_whitespace()
                .filter_map(|s| {
                    if UNIQUE_SEGMENT_COUNTS.contains(&s.len()) {
                        return Some(s);
                    }
                    None
                })
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 26);
    }
}

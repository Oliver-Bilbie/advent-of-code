use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The output is: {}", result(input));
}

fn result(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 1);
    }
}

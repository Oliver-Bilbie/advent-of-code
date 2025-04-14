use solution_2024_1_1;
use solution_2024_1_2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(year: u16, day: u16, part: u16, input: &str) -> String {
    match (year, day, part) {
        (2024, 1, 1) => solution_2024_1_1::solve(input).to_string(),
        (2024, 1, 2) => solution_2024_1_2::solve(input).to_string(),
        _ => "Not implemented yet".to_string(),
    }
}

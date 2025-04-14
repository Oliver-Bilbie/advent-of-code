use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(year: u16, day: u16, part: u16, input: &str) -> String {
    match (year, day, part) {
        _ => format!(
            "{}\n{}",
            "The wasm run script has not been auto-generated yet.",
            "Run `make auto-gen` from the project root and try again."
        ),
    }
}

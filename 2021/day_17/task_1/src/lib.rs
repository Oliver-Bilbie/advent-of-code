use wasm_bindgen::prelude::*;

fn result(input: &str) -> u64 {
    let (y_min, y_max) = input
        .trim()
        .split_once("y=")
        .unwrap()
        .1
        .split_once("..")
        .unwrap();
    let y_min = y_min.parse::<i32>().unwrap();
    let y_max = y_max.parse::<i32>().unwrap();

    // Time and space and quantized here, so applying physics isn't helpful.
    // Brute force actually seems like the most reasonable approach since the search range is so small.
    let mut max_height = 0;
    for velocity in 0..1000 {
        let mut h = 0;
        let mut v = velocity;
        let mut max_h = 0;
        while h >= y_min {
            if h <= y_max {
                max_height = max_h;
                break;
            }
            h += v;
            max_h = max_h.max(h);
            v -= 1;
        }
    }

    max_height as u64
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The highest y position is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(result(&input), 45);
    }
}

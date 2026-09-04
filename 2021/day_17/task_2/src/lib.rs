use wasm_bindgen::prelude::*;

fn result(input: &str) -> u64 {
    let (x_min, x_max) = input
        .trim()
        .split_once("x=")
        .unwrap()
        .1
        .split_once(',')
        .unwrap()
        .0
        .split_once("..")
        .unwrap();
    let x_min = x_min.parse::<i32>().unwrap();
    let x_max = x_max.parse::<i32>().unwrap();
    assert!(
        x_min >= 0 && x_max >= 0,
        "solution assumes target region is strictly in the +x direction"
    );

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
    let mut result = 0;
    for init_x in 0..500 {
        for init_y in -250..250 {
            let mut x = 0;
            let mut y = 0;
            let mut v_x = init_x;
            let mut v_y = init_y;
            while x <= x_max && y >= y_min {
                if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                    result += 1;
                    break;
                }
                x += v_x;
                v_x = (v_x - 1).max(0);
                y += v_y;
                v_y -= 1;
            }
        }
    }

    result
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "There are {} distinct initial velocity values which will hit the target",
        result(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(result(&input), 112);
    }
}

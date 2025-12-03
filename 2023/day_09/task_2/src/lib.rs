use wasm_bindgen::prelude::*;

fn get_changes(data_points: &Vec<i32>) -> Vec<i32> {
    let mut changes: Vec<i32> = Vec::new();
    for i in 0..data_points.len() - 1 {
        changes.push(data_points[i + 1] - data_points[i]);
    }
    changes
}

fn get_previous_value(data_points: &Vec<i32>) -> i32 {
    let changes = get_changes(data_points);
    let sum_of_changes = changes.iter().sum();
    match sum_of_changes {
        0 => data_points[0],
        _ => {
            let previous_step = get_previous_value(&changes);
            data_points[0] - previous_step
        }
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut previous_values_sum: i64 = 0;

    for line in input.lines() {
        let line_values = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let previous_value = get_previous_value(&line_values);
        previous_values_sum += previous_value as i64;
    }

    format!("Sum of previous values: {}", previous_values_sum)
}

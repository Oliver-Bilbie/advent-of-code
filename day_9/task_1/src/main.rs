fn get_changes(data_points: &Vec<i32>) -> Vec<i32> {
    let mut changes: Vec<i32> = Vec::new();
    for i in 0..data_points.len() - 1 {
        changes.push(data_points[i + 1] - data_points[i]);
    }
    changes
}

fn get_next_value(data_points: &Vec<i32>) -> i32 {
    let changes = get_changes(data_points);
    let sum_of_changes = changes.iter().sum();
    match sum_of_changes {
        0 => data_points[0],
        _ => {
            let final_step = get_next_value(&changes);
            data_points[data_points.len() - 1] + final_step
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("../input.txt").expect("File not found");
    let lines = file.lines().collect::<Vec<&str>>();

    let mut next_values_sum: i64 = 0;

    for line in lines {
        let line_values = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let next_value = get_next_value(&line_values);
        next_values_sum += next_value as i64;
    }

    println!("Sum of next values: {}", next_values_sum);
}

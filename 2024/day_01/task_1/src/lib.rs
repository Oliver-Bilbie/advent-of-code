pub fn solve(input: &str) -> String {
    let mut left_values = read_left_values(input);
    let mut right_values = read_right_values(input);

    left_values.sort();
    right_values.sort();

    let sum_of_differences = calculate_differences(&left_values, &right_values);

    format!("The sum of the differences is: {}", sum_of_differences)
}

fn read_left_values(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .map(|value_str| value_str.parse::<i32>().unwrap())
        .collect()
}

fn read_right_values(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|value_str| value_str.parse::<i32>().unwrap())
        .collect()
}

fn calculate_differences(left_values: &Vec<i32>, right_values: &Vec<i32>) -> u128 {
    let mut differences_sum: u128 = 0;
    for i in 0..left_values.len() {
        differences_sum += (left_values[i] - right_values[i]).abs() as u128;
    }
    differences_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> String {
        std::fs::read_to_string("../test_input.txt").unwrap()
    }

    #[test]
    fn it_reads_left_values() {
        let input = read_test_input();
        let actual_values = read_left_values(&input);
        let expected_values = vec![3, 4, 2, 1, 3, 3];
        assert_eq!(actual_values, expected_values);
    }

    #[test]
    fn it_reads_right_values() {
        let input = read_test_input();
        let actual_values = read_right_values(&input);
        let expected_values = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(actual_values, expected_values);
    }

    #[test]
    fn it_calculates_differences() {
        let input = read_test_input();
        let actual_solution = solve(&input);
        let expected_solution = "The sum of the differences is: 11";
        assert_eq!(actual_solution, expected_solution);
    }
}

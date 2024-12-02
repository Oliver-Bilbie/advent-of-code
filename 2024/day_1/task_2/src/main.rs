use std::collections::HashMap;
use std::fs;

fn main() {
    let input_string = fs::read_to_string("../input.txt").unwrap();

    let left_values = read_left_values(&input_string);
    let right_values = read_right_values(&input_string);

    let similarity_score = calculate_similarity_score(&left_values, &right_values);

    println!("The similarity score is: {}", similarity_score);
}

fn read_left_values(input: &String) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.split_whitespace().next().unwrap())
        .map(|value_str| value_str.parse::<i32>().unwrap())
        .collect()
}

fn read_right_values(input: &String) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|value_str| value_str.parse::<i32>().unwrap())
        .collect()
}

fn calculate_similarity_score(left_values: &Vec<i32>, right_values: &Vec<i32>) -> u128 {
    let right_value_counts: HashMap<i32, u128> =
        right_values.iter().fold(HashMap::new(), |mut acc, &value| {
            acc.entry(value)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            acc
        });

    left_values
        .iter()
        .map(|&l_val| {
            let count = *right_value_counts.get(&l_val).unwrap_or(&0) as u128;
            l_val as u128 * count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> String {
        fs::read_to_string("../test_input.txt").unwrap()
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
    fn it_calculates_similarity_scores() {
        let input = read_test_input();
        let left_values = read_left_values(&input);
        let right_values = read_right_values(&input);
        let actual_score = calculate_similarity_score(&left_values, &right_values);
        let expected_score = 31;
        assert_eq!(actual_score, expected_score);
    }
}

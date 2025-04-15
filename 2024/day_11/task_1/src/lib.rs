use memoize::memoize;

fn read_stones(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect()
}

fn split_stone(stone_value: u64) -> Option<Vec<u64>> {
    let stone_str = stone_value.to_string();
    let len = stone_str.len();
    if len % 2 != 0 {
        return None;
    }

    let (child_1_str, child_2_str) = stone_str.split_at(len / 2);
    let child_1 = child_1_str.parse::<u64>().unwrap();
    let child_2 = child_2_str.parse::<u64>().unwrap();

    Some(vec![child_1, child_2])
}

fn calc_children(stone_value: u64) -> Vec<u64> {
    if stone_value == 0 {
        vec![1]
    } else if let Some(children) = split_stone(stone_value) {
        children
    } else {
        vec![stone_value * 2024]
    }
}

#[memoize]
fn calc_number_of_children(stone_value: u64, num_blinks: u8) -> u64 {
    if num_blinks == 0 {
        return 1;
    }

    let children = calc_children(stone_value);
    children
        .iter()
        .map(|child| calc_number_of_children(child.clone(), num_blinks - 1))
        .sum()
}

pub fn solve(input: &str) -> String {
    let stones = read_stones(&input);
    let num_blinks = 25;
    let total_stones: u64 = stones
        .iter()
        .map(|stone| calc_number_of_children(*stone, num_blinks))
        .sum();

    format!(
        "The total number of stones after {} blinks is: {}",
        num_blinks, total_stones
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example_6() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let stones = read_stones(&input);
        let num_blinks = 6;
        let actual_stones: u64 = stones
            .iter()
            .map(|stone| calc_number_of_children(*stone, num_blinks))
            .sum();
        let expected_stones = 22;
        assert_eq!(actual_stones, expected_stones);
    }

    #[test]
    fn it_solves_the_example_25() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let stones = read_stones(&input);
        let num_blinks = 25;
        let actual_stones: u64 = stones
            .iter()
            .map(|stone| calc_number_of_children(*stone, num_blinks))
            .sum();
        let expected_stones = 55312;
        assert_eq!(actual_stones, expected_stones);
    }
}

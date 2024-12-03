use std::fs;

fn read_mul(input: &str, index: usize) -> Option<(u16, u16)> {
    let prefix = input.get(index..index + 4)?;
    if prefix != "mul(" {
        return None;
    };

    let (first, rest) = input.get(index + 4..)?.split_once(',')?;
    let (second, _) = rest.split_once(')')?;

    Some((first.parse().ok()?, second.parse().ok()?))
}

fn sum_of_products(input: &str) -> u128 {
    let mut sum_of_products: u128 = 0;

    for i in 0..input.len() {
        if let Some((a, b)) = read_mul(&input, i) {
            sum_of_products += a as u128 * b as u128;
        }
    }

    sum_of_products
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    println!("The sum of the products is: {}", sum_of_products(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> String {
        fs::read_to_string("../test_input.txt").unwrap()
    }

    #[test]
    fn it_reads_single_digits() {
        let input = "mul(1,2)";
        let expected_result = Some((1, 2));
        let actual_result = read_mul(input, 0);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_reads_triple_digits() {
        let input = "mul(123,456)";
        let expected_result = Some((123, 456));
        let actual_result = read_mul(input, 0);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_reads_with_prefix() {
        let input = "abcdmul(1,2)";
        let expected_result = Some((1, 2));
        let actual_result = read_mul(input, 4);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_reads_with_suffix() {
        let input = "mul(1,2)xyz";
        let expected_result = Some((1, 2));
        let actual_result = read_mul(input, 0);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_reads_with_prefix_and_suffix() {
        let input = "abcdmul(1,2)xyz";
        let expected_result = Some((1, 2));
        let actual_result = read_mul(input, 4);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_does_not_read_invalid() {
        let input = "mull(1,2)";
        let expected_result = None;
        let actual_result = read_mul(input, 0);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn it_solves_the_example() {
        let input = read_test_input();
        let expected_result = 161;
        let actual_result = sum_of_products(&input);
        assert_eq!(actual_result, expected_result);
    }
}

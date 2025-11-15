use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> String {
    return format!("The maximum number of bananas is: {}", max_bananas(input));
}

fn max_bananas(input: &str) -> i64 {
    let mut total_bananas = HashMap::<(i64, i64, i64, i64), i64>::new();

    for seller in input.lines() {
        let mut changes_found = HashSet::<(i64, i64, i64, i64)>::new();
        let mut secret_number = seller.parse::<i64>().unwrap();
        let mut prev_secret_number = secret_number;
        let mut changes = (0, 0, 0, 0);

        secret_number = next_secret_number(secret_number);
        changes.0 = banana_price(secret_number) - banana_price(prev_secret_number);

        prev_secret_number = secret_number;
        secret_number = next_secret_number(secret_number);
        changes.1 = banana_price(secret_number) - banana_price(prev_secret_number);

        prev_secret_number = secret_number;
        secret_number = next_secret_number(secret_number);
        changes.2 = banana_price(secret_number) - banana_price(prev_secret_number);

        prev_secret_number = secret_number;
        secret_number = next_secret_number(secret_number);
        changes.3 = banana_price(secret_number) - banana_price(prev_secret_number);

        *total_bananas.entry(changes).or_insert(0) += banana_price(secret_number);
        changes_found.insert(changes);

        for _ in 4..=2000 {
            prev_secret_number = secret_number;
            secret_number = next_secret_number(secret_number);
            changes.0 = changes.1;
            changes.1 = changes.2;
            changes.2 = changes.3;
            changes.3 = banana_price(secret_number) - banana_price(prev_secret_number);

            if !changes_found.contains(&changes) {
                *total_bananas.entry(changes).or_insert(0) += banana_price(secret_number);
                changes_found.insert(changes);
            }
        }
    }

    *total_bananas.values().max().unwrap_or(&0)
}

fn banana_price(secret_number: i64) -> i64 {
    secret_number % 10
}

fn next_secret_number(mut secret_number: i64) -> i64 {
    // Step 1
    let result = secret_number * 64;
    secret_number = mix(secret_number, result);
    secret_number = prune(secret_number);

    // Step 2
    let result = secret_number / 32;
    secret_number = mix(secret_number, result);
    secret_number = prune(secret_number);

    // Step 3
    let result = secret_number * 2048;
    secret_number = mix(secret_number, result);
    secret_number = prune(secret_number);

    return secret_number;
}

fn mix(secret_number: i64, mix_value: i64) -> i64 {
    secret_number ^ mix_value
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(max_bananas(&input), 23);
    }
}

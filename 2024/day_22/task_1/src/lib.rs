use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let result = input.lines().fold(0, |acc, l| {
        acc + nth_secret_number(l.parse().unwrap(), 2000)
    });

    return format!("The sum of 2000th secret numbers is: {}", result);
}

fn nth_secret_number(mut secret_number: u64, n: u64) -> u64 {
    for _ in 0..n {
        secret_number = next_secret_number(secret_number);
    }
    return secret_number;
}

fn next_secret_number(mut secret_number: u64) -> u64 {
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

fn mix(secret_number: u64, mix_value: u64) -> u64 {
    secret_number ^ mix_value
}

fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example_1() {
        let secret_number = 1;
        assert_eq!(nth_secret_number(secret_number, 2000), 8685429);
    }

    #[test]
    fn it_solves_the_example_2() {
        let secret_number = 10;
        assert_eq!(nth_secret_number(secret_number, 2000), 4700978);
    }

    #[test]
    fn it_solves_the_example_3() {
        let secret_number = 100;
        assert_eq!(nth_secret_number(secret_number, 2000), 15273692);
    }

    #[test]
    fn it_solves_the_example_4() {
        let secret_number = 2024;
        assert_eq!(nth_secret_number(secret_number, 2000), 8667524);
    }
}

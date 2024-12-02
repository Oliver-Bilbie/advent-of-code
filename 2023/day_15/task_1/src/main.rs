use std::fs;

fn evaluate_string_hash(input: &str) -> u32 {
    let mut current_value: u32 = 0;
    for c in input.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn main() {
    let input_string = fs::read_to_string("../input.txt").unwrap();
    let inputs: Vec<&str> = input_string[0..input_string.len() - 1].split(",").collect();

    let mut total_hash: u64 = 0;
    for input in inputs {
        total_hash += evaluate_string_hash(input) as u64;
    }
    println!("Sum of hashes: {}", total_hash);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_evaluates_example_1() {
        assert_eq!(evaluate_string_hash("rn=1"), 30);
    }

    #[test]
    fn it_evaluates_example_2() {
        assert_eq!(evaluate_string_hash("cm-"), 253);
    }

    #[test]
    fn it_evaluates_example_3() {
        assert_eq!(evaluate_string_hash("qp=3"), 97);
    }

    #[test]
    fn it_evaluates_example_4() {
        assert_eq!(evaluate_string_hash("ot=7"), 231);
    }
}

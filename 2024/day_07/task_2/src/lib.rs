use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug)]
struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => a * 10u64.pow(b.ilog10() + 1) + b,
        }
    }
}

fn read_equation(equation_str: &str) -> Equation {
    let (test_value_str, values_str) = equation_str.split_once(": ").unwrap();
    let test_value = test_value_str.parse::<u64>().unwrap();
    let values = values_str
        .split_whitespace()
        .map(|value_str| value_str.parse::<u64>().unwrap())
        .collect();
    Equation { test_value, values }
}

fn read_equations(input: &str) -> Vec<Equation> {
    input.lines().map(|line| read_equation(line)).collect()
}

fn operator_combinations(operation_count: usize) -> impl Iterator<Item = Vec<Operation>> {
    let combination_count = 3usize.pow(operation_count as u32);
    (0..combination_count).map(move |c| {
        (0..operation_count)
            .map(|o| match (c / 3usize.pow(o as u32)) % 3 {
                0 => Operation::Add,
                1 => Operation::Multiply,
                2 => Operation::Concatenate,
                _ => unreachable!(),
            })
            .collect()
    })
}

fn is_valid_equation(equation: &Equation) -> bool {
    operator_combinations(&equation.values.len() - 1).any(|operators| {
        equation.test_value
            == equation.values[1..]
                .iter()
                .zip(operators)
                .fold(equation.values[0], |acc, (value, operator)| {
                    operator.apply(acc, *value)
                })
    })
}

fn total_calibration_result(equations: &Vec<Equation>) -> u128 {
    equations
        .par_iter()
        .filter_map(|equation| {
            if is_valid_equation(equation) {
                Some(equation.test_value as u128)
            } else {
                None
            }
        })
        .sum()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let equations = read_equations(&input);
    let result = total_calibration_result(&equations);
    format!("The total calibration result is: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_example_equations() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_equations = read_equations(&input);
        let expected_equations = vec![
            Equation {
                test_value: 190,
                values: vec![10, 19],
            },
            Equation {
                test_value: 3267,
                values: vec![81, 40, 27],
            },
            Equation {
                test_value: 83,
                values: vec![17, 5],
            },
            Equation {
                test_value: 156,
                values: vec![15, 6],
            },
            Equation {
                test_value: 7290,
                values: vec![6, 8, 6, 15],
            },
            Equation {
                test_value: 161011,
                values: vec![16, 10, 13],
            },
            Equation {
                test_value: 192,
                values: vec![17, 8, 14],
            },
            Equation {
                test_value: 21037,
                values: vec![9, 7, 18, 13],
            },
            Equation {
                test_value: 292,
                values: vec![11, 6, 16, 20],
            },
        ];
        assert_eq!(actual_equations, expected_equations);
    }

    #[test]
    fn it_concatenates() {
        let concat = Operation::Concatenate;
        assert_eq!(concat.apply(1, 2), 12);
        assert_eq!(concat.apply(123, 45), 12345);
    }

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The total calibration result is: 11387";
        assert_eq!(actual_solution, expected_solution);
    }
}

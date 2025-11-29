use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Debug)]
enum Gate {
    AND,
    OR,
    XOR,
}

impl Gate {
    fn apply(&self, left: bool, right: bool) -> bool {
        match self {
            Gate::AND => left && right,
            Gate::OR => left || right,
            Gate::XOR => left ^ right,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Instruction {
    left: String,
    right: String,
    gate: Gate,
    output: String,
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The output is: {}", result(input));
}

fn result(input: &str) -> u64 {
    let mut values = read_start_values(input);
    let mut queue = read_instructions(input);
    process_queue(&mut queue, &mut values);
    read_output(&values)
}

fn read_start_values(input: &str) -> HashMap<String, bool> {
    let mut values = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let wire = line[0..3].to_string();
        let value = match &line.chars().nth(5).unwrap() {
            '1' => true,
            '0' => false,
            _ => panic!("invalid starting value"),
        };

        values.insert(wire, value);
    }

    return values;
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| {
            let v = s.split_whitespace().collect::<Vec<&str>>();

            let left = v.iter().nth(0).unwrap().to_string();
            let right = v.iter().nth(2).unwrap().to_string();
            let output = v.iter().nth(4).unwrap().to_string();
            let gate = match v.iter().nth(1).unwrap().to_owned() {
                "AND" => Gate::AND,
                "OR" => Gate::OR,
                "XOR" => Gate::XOR,
                _ => panic!("invalid logic gate"),
            };

            Instruction {
                left,
                right,
                gate,
                output,
            }
        })
        .collect::<Vec<Instruction>>()
}

fn process_queue(instructions: &mut Vec<Instruction>, values: &mut HashMap<String, bool>) {
    while !instructions.is_empty() {
        instructions.retain(|i| !process_instruction(i.clone(), values));
    }
}

fn process_instruction(instruction: Instruction, values: &mut HashMap<String, bool>) -> bool {
    let left_val = match values.get(&instruction.left) {
        Some(v) => *v,
        None => return false,
    };
    let right_val = match values.get(&instruction.right) {
        Some(v) => *v,
        None => return false,
    };
    let result = instruction.gate.apply(left_val, right_val);
    values.insert(instruction.output, result);
    return true;
}

fn read_output(values: &HashMap<String, bool>) -> u64 {
    let mut output = 0;
    let mut i = 0;

    while let Some(v) = values.get(&format!("z{:02}", i)) {
        if *v {
            output += 2_u64.pow(i);
        }
        i += 1;
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 2024);
    }
}

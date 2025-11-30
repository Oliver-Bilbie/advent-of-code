use std::collections::HashMap;
use wasm_bindgen::prelude::*;

struct Lens {
    label: String,
    focal_length: u8,
}

fn evaluate_string_hash(input: &str) -> u8 {
    let mut current_value: u32 = 0;
    for c in input.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as u8
}

fn add_lens(box_number: u8, label: String, focal_length: u8, boxes: &mut HashMap<u8, Vec<Lens>>) {
    match boxes.get_mut(&box_number) {
        Some(lenses) => {
            let mut label_found = false;
            for i in 0..lenses.len() {
                if lenses[i].label == label {
                    lenses[i].focal_length = focal_length;
                    label_found = true;
                    break;
                }
            }

            if !label_found {
                lenses.push(Lens {
                    label,
                    focal_length,
                });
            }
        }

        None => {
            boxes.insert(
                box_number,
                vec![Lens {
                    label,
                    focal_length,
                }],
            );
        }
    }
}

fn remove_lens(box_number: u8, label: String, boxes: &mut HashMap<u8, Vec<Lens>>) {
    if let Some(lenses) = boxes.get_mut(&box_number) {
        for i in 0..lenses.len() {
            if lenses[i].label == label {
                lenses.remove(i);
                break;
            }
        }
    };
}

fn calculate_focusing_power(boxes: &HashMap<u8, Vec<Lens>>) -> u64 {
    let mut focusing_power: u64 = 0;
    for (box_number, lenses) in boxes {
        for slot_number in 0..lenses.len() {
            focusing_power += (*box_number as u64 + 1)
                * (slot_number as u64 + 1)
                * lenses[slot_number].focal_length as u64;
        }
    }
    focusing_power
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let inputs: Vec<&str> = input[0..input.len() - 1].split(",").collect();

    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    for input in inputs {
        let operation_position = input
            .chars()
            .position(|c| c == '=' || c == '-')
            .expect("No operation found");
        let label = input[0..operation_position].to_string();
        let box_number = evaluate_string_hash(&label);
        let operation = input[operation_position..operation_position + 1].to_string();

        match operation.as_str() {
            "=" => {
                let focal_length = input
                    .chars()
                    .nth(operation_position + 1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as u8;
                add_lens(box_number, label, focal_length, &mut boxes)
            }
            "-" => remove_lens(box_number, label, &mut boxes),
            _ => {
                panic!("Invalid operation")
            }
        }
    }

    let focusing_power = calculate_focusing_power(&boxes);

    format!("Focusing power: {}", focusing_power)
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

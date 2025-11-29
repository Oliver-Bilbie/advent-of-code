use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Clone, Debug)]
struct Instruction {
    left: String,
    right: String,
    gate: Gate,
    output: String,
}

pub fn solve(input: &str) -> String {
    let instructions = parse_instructions(input);
    let swapped = find_swapped_wires(&instructions);

    let result = swapped.into_iter().collect::<Vec<String>>();
    format!("{}", result.join(","))
}

fn find_swapped_wires(instructions: &Vec<Instruction>) -> BTreeSet<String> {
    let mut bad_wires = BTreeSet::new();

    let max_z = instructions
        .iter()
        .filter_map(|i| match i.output.starts_with('z') {
            true => Some(i.output.clone()),
            false => None,
        })
        .max()
        .unwrap();

    for inst in instructions {
        // Output to Z must be XOR (unless it's the final carry bit)
        if inst.output.starts_with('z') && inst.gate != Gate::XOR && inst.output != max_z {
            bad_wires.insert(inst.output.clone());
        }

        // XOR gates that are NOT Z outputs must have inputs x, y
        if inst.gate == Gate::XOR
            && !inst.output.starts_with('z')
            && !is_xy_input(&inst.left, &inst.right)
        {
            bad_wires.insert(inst.output.clone());
        }

        // Intermediate XOR (x^y) must be input to another XOR
        if inst.gate == Gate::XOR && is_xy_input(&inst.left, &inst.right) {
            // Except for the very first bit (x00, y00), which doesn't have a carry_in
            if inst.left != "x00" && inst.right != "x00" {
                if !outputs_to_gate_type(&inst.output, Gate::XOR, instructions) {
                    bad_wires.insert(inst.output.clone());
                }
            }
        }

        // Intermediate AND (x&y) must be input to an OR
        if inst.gate == Gate::AND && is_xy_input(&inst.left, &inst.right) {
            // Except for x00 which feeds strictly into XOR/AND for z01 logic
            if inst.left != "x00" && inst.right != "x00" {
                if !outputs_to_gate_type(&inst.output, Gate::OR, instructions) {
                    bad_wires.insert(inst.output.clone());
                }
            }
        }
    }

    bad_wires
}

fn outputs_to_gate_type(wire: &str, target_gate: Gate, instructions: &Vec<Instruction>) -> bool {
    instructions
        .iter()
        .any(|inst| (inst.left == wire || inst.right == wire) && inst.gate == target_gate)
}

fn is_xy_input(a: &str, b: &str) -> bool {
    (a.starts_with('x') || a.starts_with('y')) && (b.starts_with('x') || b.starts_with('y'))
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| {
            let v: Vec<&str> = s.split_whitespace().collect();
            let gate = match v[1] {
                "AND" => Gate::AND,
                "OR" => Gate::OR,
                "XOR" => Gate::XOR,
                _ => panic!("invalid gate"),
            };
            Instruction {
                left: v[0].to_string(),
                right: v[2].to_string(),
                gate,
                output: v[4].to_string(),
            }
        })
        .collect()
}

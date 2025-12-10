// I definitely don't have time to go writing my own linear programming library from
// scratch today so we'll use microlp.
use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Machine {
    target: Vec<i32>,
    schema: Vec<Vec<usize>>,
}

fn parse_target(line: &str) -> Vec<i32> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let target_str = fields.last().unwrap();
    let target_str = &target_str[1..target_str.len() - 1];

    target_str
        .split(',')
        .map(|s| s.parse::<i32>().expect("value is not an integer"))
        .collect()
}

fn parse_schema(line: &str) -> Vec<Vec<usize>> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let schema_fields = &fields[1..fields.len() - 1];

    schema_fields
        .iter()
        .map(|field| {
            let inner = &field[1..field.len() - 1].trim();

            if inner.is_empty() {
                return Vec::new();
            }

            inner
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<usize>()
                        .expect("schema index is not an integer")
                })
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn parse_machine_line(line: &str) -> Machine {
    Machine {
        target: parse_target(line),
        schema: parse_schema(line),
    }
}

fn solve_machine(machine: &Machine) -> Option<i64> {
    let n_buttons = machine.schema.len();
    let n_rows = machine.target.len();

    let mut prob = Problem::new(OptimizationDirection::Minimize);
    let mut vars: Vec<Variable> = Vec::with_capacity(n_buttons);

    for j in 0..n_buttons {
        let mut max_bound: i32 = i32::MAX;
        for &r in &machine.schema[j] {
            let t = machine.target[r];
            if t < max_bound {
                max_bound = t;
            }
        }
        if max_bound == i32::MAX {
            max_bound = 0;
        }
        if max_bound < 0 {
            // If any target is negative, that row is impossible.
            return None;
        }

        let var = prob.add_integer_var(1.0, (0, max_bound));
        vars.push(var);
    }

    for i in 0..n_rows {
        let mut lhs: Vec<(Variable, f64)> = Vec::new();
        for j in 0..n_buttons {
            if machine.schema[j].contains(&i) {
                lhs.push((vars[j], 1.0));
            }
        }

        let rhs = machine.target[i] as f64;

        if lhs.is_empty() {
            // No button affects this row; must be 0 in target
            if rhs != 0.0 {
                return None;
            }
        } else {
            prob.add_constraint(&lhs, ComparisonOp::Eq, rhs);
        }
    }

    let solution = match prob.solve() {
        Ok(sol) => sol,
        Err(_) => return None,
    };

    let total_presses = solution.iter().map(|(_, n)| n.round() as i64).sum();

    Some(total_presses)
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "Joltages are configured after {} button presses",
        result(input)
    );
}

fn result(input: &str) -> u64 {
    let mut total: u64 = 0;
    for (n, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let machine = parse_machine_line(line);

        match solve_machine(&machine) {
            Some(presses) => {
                total += presses as u64;
            }
            None => {
                eprintln!("Line {}: no feasible solution", n + 1);
            }
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 33);
    }
}

use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Debug)]
struct Machine {
    a: Position,
    b: Position,
    prize: Position,
}

#[derive(PartialEq, Debug)]
struct Solution {
    a_presses: i64,
    b_presses: i64,
}

impl Solution {
    fn get_cost(&self) -> u64 {
        (3 * self.a_presses + self.b_presses) as u64
    }
}

fn read_values(line: &str) -> Position {
    let (_, x_str) = line.split_once("X").unwrap();
    let (mut x_str, rest) = x_str.split_once(",").unwrap();
    let (_, mut y_str) = rest.split_once("Y").unwrap();

    x_str = &x_str[1..];
    y_str = &y_str[1..];

    let x = x_str.parse::<i64>().unwrap();
    let y = y_str.parse::<i64>().unwrap();

    Position { x, y }
}

fn read_machines(input: &str) -> Vec<Machine> {
    let lines: Vec<&str> = input.lines().collect();
    let mut machines: Vec<Machine> = Vec::with_capacity(lines.len() / 4);
    let mut i = 0;

    while lines.get(i).is_some() {
        let a = read_values(lines[i]);
        let b = read_values(lines[i + 1]);
        let mut prize = read_values(lines[i + 2]);
        prize.x += 10000000000000;
        prize.y += 10000000000000;
        machines.push(Machine { a, b, prize });
        i += 4;
    }

    machines
}

fn safe_divide(numerator: i64, denominator: i64) -> Option<i64> {
    if denominator == 0 {
        None
    } else if numerator % denominator == 0 {
        Some(numerator / denominator)
    } else {
        None
    }
}

fn solve_machine(machine: &Machine) -> Option<Solution> {
    let a_presses = safe_divide(
        machine.prize.x * machine.b.y - machine.prize.y * machine.b.x,
        machine.a.x * machine.b.y - machine.a.y * machine.b.x,
    )?;
    if a_presses < 0 {
        return None;
    }

    let b_presses = safe_divide(machine.prize.y - a_presses * machine.a.y, machine.b.y)?;
    if b_presses < 0 {
        return None;
    }

    Some(Solution {
        a_presses,
        b_presses,
    })
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let machines = read_machines(&input);
    let total_cost: u64 = machines
        .iter()
        .filter_map(|machine| solve_machine(machine))
        .map(|solution| solution.get_cost())
        .sum();

    format!("The total cost is: {}", total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example_case_1() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[0]);
        assert!(actual_solution.is_none());
    }

    #[test]
    fn it_solves_example_case_2() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[1]);
        assert!(actual_solution.is_some());
    }

    #[test]
    fn it_solves_example_case_3() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[2]);
        assert!(actual_solution.is_none());
    }

    #[test]
    fn it_solves_example_case_4() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[3]);
        assert!(actual_solution.is_some());
    }
}

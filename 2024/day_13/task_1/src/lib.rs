#[derive(PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug)]
struct Machine {
    a: Position,
    b: Position,
    prize: Position,
}

#[derive(PartialEq, Debug)]
struct Solution {
    a_presses: i32,
    b_presses: i32,
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

    let x = x_str.parse::<i32>().unwrap();
    let y = y_str.parse::<i32>().unwrap();

    Position { x, y }
}

fn read_machines(input: &str) -> Vec<Machine> {
    let lines: Vec<&str> = input.lines().collect();
    let mut machines: Vec<Machine> = Vec::with_capacity(lines.len() / 4);
    let mut i = 0;

    while lines.get(i).is_some() {
        let a = read_values(lines[i]);
        let b = read_values(lines[i + 1]);
        let prize = read_values(lines[i + 2]);
        machines.push(Machine { a, b, prize });
        i += 4;
    }

    machines
}

fn safe_divide(numerator: i32, denominator: i32) -> Option<i32> {
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
    if a_presses < 0 || a_presses > 100 {
        return None;
    }

    let b_presses = safe_divide(machine.prize.y - a_presses * machine.a.y, machine.b.y)?;
    if b_presses < 0 || b_presses > 100 {
        return None;
    }

    Some(Solution {
        a_presses,
        b_presses,
    })
}

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
        let expected_solution = Some(Solution {
            a_presses: 80,
            b_presses: 40,
        });
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn it_solves_example_case_2() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[1]);
        let expected_solution = None;
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn it_solves_example_case_3() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[2]);
        let expected_solution = Some(Solution {
            a_presses: 38,
            b_presses: 86,
        });
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn it_solves_example_case_4() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let machines = read_machines(&input);
        let actual_solution = solve_machine(&machines[3]);
        let expected_solution = None;
        assert_eq!(actual_solution, expected_solution);
    }

    #[test]
    fn it_solves_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The total cost is: 480";
        assert_eq!(actual_solution, expected_solution);
    }
}

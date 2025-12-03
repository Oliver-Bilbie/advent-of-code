use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let reports = read_reports(&input);
    let safe_reports = evaluate_safe_report_count(&reports);
    format!("The number of safe reports is: {}", safe_reports)
}

fn drop_level(report: &[i32], level: usize) -> Vec<i32> {
    let mut damped_report = Vec::with_capacity(report.len() - 1);
    damped_report.extend(&report[..level]);
    damped_report.extend(&report[level + 1..]);
    damped_report
}

fn read_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value_str| value_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn find_first_unsafe_index(report: &[i32]) -> Option<usize> {
    // Calculate the differences between adjacent pairs of elements
    let mut diffs_iter = report.windows(2).map(|v| v[1] - v[0]).enumerate();

    // Detemine whether the numbers are increasing or decreasing
    // We must also ensure that the first difference is valid
    let is_increasing = match diffs_iter.next()? {
        (_, diff) if (1..=3).contains(&diff) => true,
        (_, diff) if (-3..=-1).contains(&diff) => false,
        _ => return Some(1),
    };

    // Validate the remaining differences
    for (index, diff) in diffs_iter {
        match is_increasing {
            true if !(1..=3).contains(&diff) => return Some(index + 1),
            false if !(-3..=-1).contains(&diff) => return Some(index + 1),
            _ => {}
        }
    }
    None
}

fn is_safe_with_damping(report: &[i32]) -> bool {
    if let Some(unsafe_index) = find_first_unsafe_index(report) {
        (0..=unsafe_index).any(|damping_index| {
            let dampened_report = drop_level(&report, damping_index);
            find_first_unsafe_index(&dampened_report).is_none()
        })
    } else {
        true
    }
}

fn evaluate_safe_report_count(reports: &[Vec<i32>]) -> u128 {
    reports
        .iter()
        .filter(|report| is_safe_with_damping(&report))
        .count() as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_input() -> String {
        std::fs::read_to_string("../test_input.txt").unwrap()
    }

    #[test]
    fn it_reads_reports() {
        let input = read_test_input();
        let actual_values = read_reports(&input);
        let expected_values = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(actual_values, expected_values);
    }

    #[test]
    fn it_damps_the_first_level() {
        let input = vec![3, 1, 2, 3, 4];
        assert_eq!(is_safe_with_damping(&input), true);
    }

    #[test]
    fn it_damps_the_final_level() {
        let input = vec![1, 2, 3, 4, 1];
        assert_eq!(is_safe_with_damping(&input), true);
    }

    #[test]
    fn it_damps_the_third_level() {
        let input = vec![1, 2, 0, 4, 5];
        assert_eq!(is_safe_with_damping(&input), true);
    }

    #[test]
    fn it_damps_only_one_level() {
        let input = vec![1, 2, 0, 0, 3, 4, 5];
        assert_eq!(is_safe_with_damping(&input), false);
    }

    #[test]
    fn it_solves_the_example() {
        let input = read_test_input();
        let actual_solution = solve(&input);
        let expected_solution = "The number of safe reports is: 4";
        assert_eq!(actual_solution, expected_solution);
    }
}

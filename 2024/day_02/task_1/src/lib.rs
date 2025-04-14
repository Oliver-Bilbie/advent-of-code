pub fn solve(input: &str) -> String {
    let reports = read_reports(&input);
    let safe_reports = evaluate_safe_report_count(&reports);
    format!("The number of safe reports is: {}", safe_reports)
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

fn is_safe(report: &Vec<i32>) -> bool {
    // Calculate the differences between adjacent pairs of elements
    let mut diffs_iter = report.windows(2).map(|v| v[1] - v[0]);

    // Detemine whether the numbers are increasing or decreasing
    // We must also ensure that the first difference is valid
    let is_increasing = match diffs_iter.next() {
        Some(diff) => match diff {
            diff if (1..=3).contains(&diff) => true,
            diff if (-3..=-1).contains(&diff) => false,
            _ => return false,
        },
        None => true,
    };

    for diff in diffs_iter {
        match is_increasing {
            true if !(1..=3).contains(&diff) => return false,
            false if !(-3..=-1).contains(&diff) => return false,
            _ => {}
        }
    }
    true
}

fn evaluate_safe_report_count(reports: &Vec<Vec<i32>>) -> u128 {
    reports.iter().filter(|report| is_safe(report)).count() as u128
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
    fn it_solves_the_example() {
        let input = read_test_input();
        let actual_solution = solve(&input);
        let expected_solution = "The number of safe reports is: 2";
        assert_eq!(actual_solution, expected_solution);
    }
}

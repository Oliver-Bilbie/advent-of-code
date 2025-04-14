fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let safe_reports = solution_2024_02_1::solve(&input);
    println!("The number of safe reports is: {}", safe_reports);
}

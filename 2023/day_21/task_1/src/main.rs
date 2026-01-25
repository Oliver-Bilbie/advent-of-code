fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let solution = solution_2023_21_1::solve(&input);
    println!("{}", solution);
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let solution = solution_2023_22_2::solve(&input);
    println!("{}", solution);
}

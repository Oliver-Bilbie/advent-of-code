fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let solution = solution_2024_20_2::solve(&input);
    println!("{}", solution);
}

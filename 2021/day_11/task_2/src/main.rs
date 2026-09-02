fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let solution = solution_2021_11_2::solve(&input);
    println!("{}", solution);
}

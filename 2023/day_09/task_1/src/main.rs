fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("{}", solution_2023_09_1::solve(&input));
}

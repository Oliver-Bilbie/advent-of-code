fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("{}", solution_2023_17_2::solve(&input));
}

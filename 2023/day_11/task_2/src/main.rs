fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    println!("{}", solution_2023_11_2::solve(&input));
}

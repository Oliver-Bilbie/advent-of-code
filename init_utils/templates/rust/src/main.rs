fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let solution = __SOLUTION_NAME__::solve(&input);
    println!("{}", solution);
}

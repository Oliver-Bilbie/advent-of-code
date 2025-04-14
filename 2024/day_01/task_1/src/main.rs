use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    println!("{}", solution_2024_01_1::solve(&input));
}

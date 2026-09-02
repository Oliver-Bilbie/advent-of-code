use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

impl Bracket {
    fn from_char(c: char) -> Option<Bracket> {
        match c {
            '(' => Some(Bracket::RoundOpen),
            ')' => Some(Bracket::RoundClose),
            '[' => Some(Bracket::SquareOpen),
            ']' => Some(Bracket::SquareClose),
            '{' => Some(Bracket::CurlyOpen),
            '}' => Some(Bracket::CurlyClose),
            '<' => Some(Bracket::AngleOpen),
            '>' => Some(Bracket::AngleClose),
            _ => None,
        }
    }

    fn score(&self) -> Option<u64> {
        match self {
            Bracket::RoundClose => Some(3),
            Bracket::SquareClose => Some(57),
            Bracket::CurlyClose => Some(1197),
            Bracket::AngleClose => Some(25137),
            _ => None,
        }
    }

    fn is_open(&self) -> bool {
        self.clone() as u8 % 2 == 0
    }
}

fn is_pair(open: &Bracket, close: &Bracket) -> bool {
    matches!(
        (open, close),
        (Bracket::RoundOpen, Bracket::RoundClose)
            | (Bracket::SquareOpen, Bracket::SquareClose)
            | (Bracket::CurlyOpen, Bracket::CurlyClose)
            | (Bracket::AngleOpen, Bracket::AngleClose)
    )
}

fn corruption_score(line: &str) -> Option<u64> {
    let mut stack = Vec::<Bracket>::new();
    for c in line.chars() {
        let next = Bracket::from_char(c).expect("char is not a bracket");
        if next.is_open() {
            stack.push(next);
            continue;
        }
        match stack.pop() {
            Some(prev) => {
                if !is_pair(&prev, &next) {
                    return next.score();
                }
            }
            None => {
                return next.score();
            }
        }
    }
    None
}

fn result(input: &str) -> u64 {
    input
        .lines()
        .map(|l| corruption_score(l).unwrap_or(0))
        .sum()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The syntax error score is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 26397);
    }
}

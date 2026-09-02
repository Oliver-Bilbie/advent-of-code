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
            Bracket::RoundOpen => Some(1),
            Bracket::SquareOpen => Some(2),
            Bracket::CurlyOpen => Some(3),
            Bracket::AngleOpen => Some(4),
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

fn autocomplete_score(line: &str) -> Option<u64> {
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
                    return None;
                }
            }
            None => {
                return None;
            }
        }
    }

    let mut score = 0;
    while let Some(b) = stack.pop() {
        score = 5 * score + b.score().unwrap();
    }
    Some(score)
}

fn result(input: &str) -> u64 {
    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|l| autocomplete_score(l))
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The middle score is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 288957);
    }
}

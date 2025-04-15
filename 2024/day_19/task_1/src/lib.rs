use rayon::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(c: char) -> Color {
        match c {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => panic!("invalid color"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Towel {
    stripes: Vec<Color>,
}

impl Towel {
    fn len(&self) -> usize {
        self.stripes.len()
    }

    fn fits_pattern(&self, pattern: &[Color]) -> bool {
        if let Some(pattern_start) = pattern.get(0..self.len()) {
            self.stripes == pattern_start
        } else {
            false
        }
    }
}

impl FromIterator<Color> for Towel {
    fn from_iter<I: IntoIterator<Item = Color>>(iter: I) -> Self {
        let stripes = iter.into_iter().collect();
        Towel { stripes }
    }
}

struct Input {
    towels: HashSet<Towel>,
    patterns: Vec<Vec<Color>>,
}

impl Input {
    fn from_str(input: &str) -> Input {
        let towels = input
            .lines()
            .nth(0)
            .unwrap()
            .split(", ")
            .map(|towel_str| towel_str.chars().map(|c| Color::from_char(c)).collect())
            .collect();

        let patterns = input
            .lines()
            .skip(2)
            .map(|pattern_str| pattern_str.chars().map(|c| Color::from_char(c)).collect())
            .collect();

        Input { towels, patterns }
    }
}

fn pattern_is_valid(pattern: &[Color], towels: &HashSet<Towel>) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    let next_towels: Vec<&Towel> = towels
        .iter()
        .filter(|towel| towel.fits_pattern(pattern))
        .collect();

    if next_towels.len() == 0 {
        return false;
    }

    next_towels
        .iter()
        .find(|&&towel| pattern_is_valid(&pattern[towel.len()..], towels))
        .is_some()
}

pub fn solve(input_str: &str) -> String {
    let input = Input::from_str(&input_str);
    let valid_patterns = input
        .patterns
        .par_iter()
        .filter(|pattern| pattern_is_valid(pattern, &input.towels))
        .count();
    format!("There are {} valid patterns", valid_patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_input() {
        let input_str = std::fs::read_to_string("../test_input.txt").unwrap();
        let input = Input::from_str(&input_str);

        let expected_towels = HashSet::from([
            Towel {
                stripes: vec![Color::Red],
            },
            Towel {
                stripes: vec![Color::White, Color::Red],
            },
            Towel {
                stripes: vec![Color::Black],
            },
            Towel {
                stripes: vec![Color::Green],
            },
            Towel {
                stripes: vec![Color::Black, Color::White, Color::Blue],
            },
            Towel {
                stripes: vec![Color::Red, Color::Black],
            },
            Towel {
                stripes: vec![Color::Green, Color::Black],
            },
            Towel {
                stripes: vec![Color::Black, Color::Red],
            },
        ]);

        let expected_patterns = vec![
            vec![
                Color::Black,
                Color::Red,
                Color::White,
                Color::Red,
                Color::Red,
            ],
            vec![Color::Black, Color::Green, Color::Green, Color::Red],
            vec![Color::Green, Color::Black, Color::Black, Color::Red],
            vec![
                Color::Red,
                Color::Red,
                Color::Black,
                Color::Green,
                Color::Black,
                Color::Red,
            ],
            vec![Color::Blue, Color::Black, Color::White, Color::Blue],
            vec![
                Color::Black,
                Color::White,
                Color::Blue,
                Color::Red,
                Color::Red,
                Color::Green,
            ],
            vec![Color::Black, Color::Red, Color::Green, Color::Red],
            vec![
                Color::Black,
                Color::Black,
                Color::Red,
                Color::Green,
                Color::White,
                Color::Black,
            ],
        ];

        assert_eq!(input.towels, expected_towels);
        assert_eq!(input.patterns, expected_patterns);
    }

    #[test]
    fn it_solves_the_example() {
        let input_str = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input_str);
        let expected_solution = "There are 6 valid patterns";
        assert_eq!(actual_solution, expected_solution);
    }
}

use wasm_bindgen::prelude::*;

struct Dice {
    value: u32,
    roll_count: u64,
}
impl Dice {
    fn new() -> Dice {
        Dice {
            value: 1,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        let result = self.value;
        if self.value < 100 {
            self.value += 1;
        } else {
            self.value = 1;
        }
        self.roll_count += 1;
        result
    }
}

struct Player {
    position: u32, // 0-indexed
    score: u64,
}
impl Player {
    fn new(position: u32) -> Player {
        Player {
            position: position - 1,
            score: 0,
        }
    }

    fn take_turn(&mut self, distance: u32) {
        self.position = (self.position + distance) % 10;
        self.score += self.position as u64 + 1; // score uses 1-indexed position
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }

    fn score(&self) -> u64 {
        self.score
    }
}

fn result(input: &str) -> u64 {
    let start_positions: Vec<u32> = input
        .lines()
        .map(|ln| {
            let (_, pos) = ln.split_once(": ").unwrap();
            pos.parse().unwrap()
        })
        .collect();

    let mut dice = Dice::new();
    let mut p1 = Player::new(start_positions[0]);
    let mut p2 = Player::new(start_positions[1]);

    loop {
        let distance = dice.roll() + dice.roll() + dice.roll();
        p1.take_turn(distance);
        if p1.has_won() {
            return p2.score() * dice.roll_count;
        }

        let distance = dice.roll() + dice.roll() + dice.roll();
        p2.take_turn(distance);
        if p2.has_won() {
            return p1.score() * dice.roll_count;
        }
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The result is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 739785);
    }
}

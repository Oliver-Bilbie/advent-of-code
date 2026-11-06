use wasm_bindgen::prelude::*;

struct Roll {
    value: u32,
    ways: u64,
}

const ROLLS: [Roll; 7] = [
    Roll { value: 3, ways: 1 },
    Roll { value: 4, ways: 3 },
    Roll { value: 5, ways: 6 },
    Roll { value: 6, ways: 7 },
    Roll { value: 7, ways: 6 },
    Roll { value: 8, ways: 3 },
    Roll { value: 9, ways: 1 },
];

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

    fn take_turn(&self, distance: u32) -> Player {
        let position = (self.position + distance) % 10;
        let score = self.score + position as u64 + 1; // score uses 1-indexed position
        Player { position, score }
    }

    fn has_won(&self) -> bool {
        self.score >= 21
    }
}

fn dfs_p1(p1: &Player, p2: &Player, total_wins: &mut [u64; 2], equivalent_states: u64) {
    if p2.has_won() {
        total_wins[1] += equivalent_states;
        return;
    }
    for roll in ROLLS {
        dfs_p2(
            &p1.take_turn(roll.value),
            p2,
            total_wins,
            equivalent_states * roll.ways,
        );
    }
}

fn dfs_p2(p1: &Player, p2: &Player, total_wins: &mut [u64; 2], equivalent_states: u64) {
    if p1.has_won() {
        total_wins[0] += equivalent_states;
        return;
    }
    for roll in ROLLS {
        dfs_p1(
            p1,
            &p2.take_turn(roll.value),
            total_wins,
            equivalent_states * roll.ways,
        );
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

    let p1 = Player::new(start_positions[0]);
    let p2 = Player::new(start_positions[1]);
    let mut total_wins: [u64; 2] = [0, 0];

    dfs_p1(&p1, &p2, &mut total_wins, 1);
    *total_wins.iter().max().unwrap()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The output is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 444356092776315);
    }
}

use wasm_bindgen::prelude::*;

const MIN_HEIGHT: u16 = 1;

#[derive(Clone, Debug)]
struct Range {
    start: u16,
    end: u16,
}

impl Range {
    fn new(point_1: u16, point_2: u16) -> Range {
        if point_1 <= point_2 {
            Range {
                start: point_1,
                end: point_2,
            }
        } else {
            Range {
                start: point_2,
                end: point_1,
            }
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        if self.start <= other.start {
            self.end >= other.start
        } else {
            other.end >= self.start
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    id: usize,
    x: Range,
    y: Range,
    z: Range,
}

struct State {
    values: Vec<Block>,
}

impl State {
    fn new() -> State {
        State { values: vec![] }
    }

    fn add_block(&mut self, x: Range, y: Range, z: Range) -> &Block {
        let pos = self
            .values
            .iter()
            .position(|b| b.z.start > z.start || (b.z.start == z.start && b.z.end >= z.end))
            .unwrap_or(self.values.len());
        let blk = Block {
            id: self.values.len(),
            x,
            y,
            z: z.clone(),
        };
        self.values.insert(pos, blk);
        &self.values[pos]
    }

    fn unpause_time(&mut self) -> bool {
        let can_fall: Vec<usize> = self
            .values
            .iter()
            .enumerate()
            .filter_map(|(i, falling_blk)| {
                if falling_blk.z.start == MIN_HEIGHT {
                    return None;
                }
                let has_support = self
                    .values
                    .iter()
                    .find(|b| {
                        b.z.end + 1 == falling_blk.z.start
                            && b.x.overlaps(&falling_blk.x)
                            && b.y.overlaps(&falling_blk.y)
                    })
                    .is_some();
                if has_support {
                    return None;
                }
                Some(i)
            })
            .collect();
        for i in &can_fall {
            let z = &mut self.values.get_mut(*i).unwrap().z;
            z.start -= 1;
            z.end -= 1;
        }
        !can_fall.is_empty()
    }
}

fn read_position(value: &str) -> [u16; 3] {
    value
        .split(',')
        .map(|s| s.parse().expect("not a u16"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("expected exactly 3 coordinates")
}

fn read_input(input: &str) -> State {
    let mut state = State::new();
    for line in input.lines() {
        let (p1, p2) = line
            .split_once('~')
            .expect("input should contain '~' delimiter");
        let [x1, y1, z1] = read_position(p1);
        let [x2, y2, z2] = read_position(p2);
        state.add_block(Range::new(x1, x2), Range::new(y1, y2), Range::new(z1, z2));
    }
    state
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("{} bricks can safely be disintegrated", result(input));
}

fn result(input: &str) -> u64 {
    let mut state = read_input(input);
    // Let blocks fall until all have settled
    while state.unpause_time() {}
    // Simulate the disintegration of each block to see if anything moves
    let can_remove: Vec<Block> = state
        .values
        .iter()
        .filter_map(|b| {
            let mut s = State {
                values: state
                    .values
                    .iter()
                    .filter(|v| v.id != b.id)
                    .cloned()
                    .collect(),
            };
            match s.unpause_time() {
                true => None,
                false => Some(b.clone()),
            }
        })
        .collect();
    return can_remove.len() as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 5);
    }
}

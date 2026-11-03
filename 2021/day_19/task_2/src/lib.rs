use std::collections::{HashMap, HashSet, VecDeque};
use wasm_bindgen::prelude::*;

const MIN_OVERLAP: usize = 12;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}
impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    fn min(p1: &Position, p2: &Position) -> Position {
        if p1 <= p2 { p1 } else { p2 }.clone()
    }

    fn diff(p1: &Position, p2: &Position) -> Position {
        if p1 >= p2 { p1.sub(p2) } else { p2.sub(p1) }
    }

    fn add(&self, offset: &Position) -> Position {
        Position {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }

    fn sub(&self, offset: &Position) -> Position {
        Position {
            x: self.x - offset.x,
            y: self.y - offset.y,
            z: self.z - offset.z,
        }
    }

    fn r2(&self) -> u64 {
        (self.x * self.x) as u64 + (self.y * self.y) as u64 + (self.z * self.z) as u64
    }

    fn rotate(&self, n: u8) -> Position {
        if n >= 24 {
            panic!("Invalid degree of rotation (must be in range 0..24)");
        }

        let perm = n % 6;
        let sign = n / 6;
        let (x, y, z) = match (perm < 3, sign) {
            (true, 0) => (self.x, self.y, self.z),
            (true, 1) => (self.x, -self.y, -self.z),
            (true, 2) => (-self.x, self.y, -self.z),
            (true, 3) => (-self.x, -self.y, self.z),
            (false, 0) => (-self.x, -self.y, -self.z),
            (false, 1) => (-self.x, self.y, self.z),
            (false, 2) => (self.x, -self.y, self.z),
            (false, 3) => (self.x, self.y, -self.z),
            _ => unreachable!(),
        };

        match perm {
            0 => Position { x, y, z },          // xyz
            1 => Position { x: y, y: z, z: x }, // yzx
            2 => Position { x: z, y: x, z: y }, // zxy
            3 => Position { x, y: z, z: y },    // xzy
            4 => Position { x: y, y: x, z },    // yxz
            5 => Position { x: z, y, z: x },    // zyx
            _ => unreachable!(),
        }
    }

    fn manhattan(&self, other: &Position) -> u64 {
        let x = (self.x - other.x).abs() as u64;
        let y = (self.y - other.y).abs() as u64;
        let z = (self.z - other.z).abs() as u64;
        x + y + z
    }
}

struct Beacons {
    beacons: HashSet<Position>,
    offsets: HashMap<Position, HashSet<Position>>,
}
impl Beacons {
    fn new() -> Beacons {
        Beacons {
            beacons: HashSet::new(),
            offsets: HashMap::new(),
        }
    }

    fn insert(&mut self, pos: Position) {
        for b in &self.beacons {
            self.offsets
                .entry(Position::diff(b, &pos))
                .or_default()
                .insert(Position::min(b, &pos));
        }
        self.beacons.insert(pos);
    }

    fn offset(&self, diffs: &Vec<(Position, Position)>) -> Option<Position> {
        let map_pairs: Vec<(&Position, &Position, &HashSet<Position>)> = diffs
            .iter()
            .filter_map(|(start_pos, diff)| match self.offsets.get(diff) {
                Some(true_pos) => Some((start_pos, diff, true_pos)),
                None => None,
            })
            .collect();

        let mut offset_beacons = HashMap::<Position, HashSet<Position>>::new();
        for (start_pos, diff, true_pos) in map_pairs {
            for p in true_pos {
                offset_beacons
                    .entry(p.sub(start_pos))
                    .or_default()
                    .extend([p.clone(), p.add(diff)]);
            }
        }
        for (offset, beacons) in offset_beacons {
            if beacons.len() >= MIN_OVERLAP {
                return Some(offset);
            }
        }
        None
    }
}

fn read_scanners(input: &str) -> VecDeque<Vec<Position>> {
    let mut scanners = VecDeque::new();
    let mut positions = Vec::<Position>::new();

    for ln in input.lines() {
        if ln.is_empty() {
            positions.sort_by(|a, b| a.r2().cmp(&b.r2()));
            scanners.push_back(positions);
            positions = Vec::new();
        }

        let items = ln.splitn(3, ',');
        let pos: Vec<i32> = items.filter_map(|s| s.parse::<i32>().ok()).collect();
        // this check filters out "--- scanner n ---" lines
        if pos.len() == 3 {
            positions.push(Position::new(pos[0], pos[1], pos[2]));
        }
    }
    scanners.push_back(positions);

    scanners
}

fn calculate_diffs(s: &Vec<Position>) -> Vec<(Position, Position)> {
    let mut diffs = Vec::new();
    for (i, p1) in s[..s.len() - 1].iter().enumerate() {
        for p2 in s[i..].iter() {
            diffs.push((Position::min(p1, p2), Position::diff(p1, p2)));
        }
    }
    diffs
}

fn rotate_positions(positions: &Vec<Position>, n: u8) -> Vec<Position> {
    positions.iter().map(|p| p.rotate(n)).collect()
}

fn result(input: &str) -> u64 {
    let mut scanners = read_scanners(input);
    let mut beacons = Beacons::new();
    let mut scanner_pos = Vec::new();

    for b in scanners.pop_front().expect("No scanners were found") {
        beacons.insert(b);
    }

    while let Some(scanner) = scanners.pop_front() {
        let mut is_found = false;
        for direction in 0..24 {
            let s = rotate_positions(&scanner, direction);
            let diffs = calculate_diffs(&s);
            if let Some(offset) = beacons.offset(&diffs) {
                scanner_pos.push(offset.clone());
                for pos in s {
                    beacons.insert(pos.add(&offset));
                }
                is_found = true;
                break;
            }
        }
        if !is_found {
            scanners.push_back(scanner);
        }
    }

    let mut max_distance = 0;
    for (i, p1) in scanner_pos[..scanner_pos.len() - 1].iter().enumerate() {
        for p2 in scanner_pos[i..].iter() {
            max_distance = max_distance.max(p1.manhattan(p2));
        }
    }
    max_distance
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "The largest Manhattan distance between two scanners is {}",
        result(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 3621);
    }
}

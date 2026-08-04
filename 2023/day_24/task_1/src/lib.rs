use aoc_utils::position::Position;
use nalgebra::{Matrix2, Matrix2x1};
use wasm_bindgen::prelude::*;

struct Hailstone {
    position: Position<i64>,
    velocity: Position<i64>,
}

impl Hailstone {
    fn from_str(input: &str) -> Hailstone {
        let read_position = |value: &str| -> [i64; 3] {
            value
                .split(',')
                .map(|s| s.trim().parse().expect("not a i64"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("expected exactly 3 coordinates")
        };
        let (p, v) = input
            .split_once('@')
            .expect("input should contain '@' delimiter");
        let [x, y, _z] = read_position(p);
        let [vx, vy, _vz] = read_position(v);
        Hailstone {
            position: Position { row: x, column: y },
            velocity: Position {
                row: vx,
                column: vy,
            },
        }
    }
}

fn find_intersection(h1: &Hailstone, h2: &Hailstone) -> Option<Position<f64>> {
    let p1 = &h1.position;
    let p2 = &h2.position;
    let v1 = &h1.velocity;
    let v2 = &h2.velocity;

    let position_difs = Matrix2x1::new((p2.row - p1.row) as f64, (p2.column - p1.column) as f64);
    #[rustfmt::skip]
    let velocities = Matrix2::new(
        v1.row as f64, -v2.row as f64,
        v1.column as f64, -v2.column as f64,
    );
    let inv_velocities = match velocities.try_inverse() {
        Some(v) => v,
        None => return None,
    };
    let int_times = inv_velocities * position_difs;

    if int_times[0].is_sign_negative() || int_times[1].is_sign_negative() {
        return None;
    }

    Some(Position {
        row: p1.row as f64 + int_times[0] * v1.row as f64,
        column: p1.column as f64 + int_times[0] * v1.column as f64,
    })
}

fn check_bounds(position: Position<f64>, search_min: f64, search_max: f64) -> bool {
    position.row >= search_min
        && position.column >= search_min
        && position.row <= search_max
        && position.column <= search_max
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    const SEARCH_MIN: f64 = 200000000000000_f64;
    const SEARCH_MAX: f64 = 400000000000000_f64;
    return format!(
        "{} intersections occur within the test area",
        result(input, SEARCH_MIN, SEARCH_MAX)
    );
}

fn result(input: &str, search_min: f64, search_max: f64) -> u64 {
    let hailstones: Vec<Hailstone> = input.lines().map(|l| Hailstone::from_str(l)).collect();
    let mut int_count = 0;

    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i + 1) {
            if let Some(p) = find_intersection(h1, h2) {
                if check_bounds(p, search_min, search_max) {
                    int_count += 1
                }
            }
        }
    }

    return int_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        const SEARCH_MIN: f64 = 7_f64;
        const SEARCH_MAX: f64 = 27_f64;
        assert_eq!(result(&input, SEARCH_MIN, SEARCH_MAX), 2);
    }

    #[test]
    fn it_finds_intersection() {
        let h1 = Hailstone {
            position: Position {
                row: 19,
                column: 13,
            },
            velocity: Position { row: -2, column: 1 },
        };
        let h2 = Hailstone {
            position: Position {
                row: 18,
                column: 19,
            },
            velocity: Position {
                row: -1,
                column: -1,
            },
        };
        assert_eq!(
            find_intersection(&h1, &h2),
            Some(Position::<f64> {
                row: 14.333333333333332,
                column: 15.333333333333334,
            })
        );
    }
}

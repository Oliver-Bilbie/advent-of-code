use nalgebra::{Matrix3, Vector3};
use wasm_bindgen::prelude::*;

struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Hailstone {
    fn from_str(input: &str) -> Hailstone {
        let read_position = |value: &str| -> [f64; 3] {
            value
                .split(',')
                .map(|s| s.trim().parse().expect("not a f64"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("expected exactly 3 coordinates")
        };
        let (p, v) = input
            .split_once('@')
            .expect("input should contain '@' delimiter");
        let [x, y, z] = read_position(p);
        let [vx, vy, vz] = read_position(v);
        Hailstone {
            position: Vector3::new(x, y, z),
            velocity: Vector3::new(vx, vy, vz),
        }
    }
}

struct Pairs {
    values: [(usize, usize); 3],
}

impl Pairs {
    fn new() -> Pairs {
        Pairs {
            values: [(0, 1), (0, 2), (1, 2)],
        }
    }

    fn next(&mut self) {
        for p in &mut self.values {
            *p = (p.0 + 1, p.1 + 1);
        }
    }

    fn get(&self) -> &[(usize, usize); 3] {
        &self.values
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The sum of coordinates is: {}", result(input));
}

fn result(input: &str) -> u64 {
    // based on some painful vector algebra, we construct a system of equations in the form
    // a•P = b
    // where P is the stone's initial position vector
    //       a is a 3x3 matrix
    //       b is a 3x1 matrix

    let hailstones: Vec<Hailstone> = input.lines().map(|l| Hailstone::from_str(l)).collect();
    let mut pairs = Pairs::new();

    // some pairs don't play nicely due to the limits of f64s, so we will just keep trying different pairs until something works
    loop {
        let mut a_rows = Vec::<Vector3<f64>>::with_capacity(3);
        let mut b_terms = Vec::<f64>::with_capacity(3);

        for p in pairs.get() {
            let h_i = &hailstones.get(p.0).expect("no valid solution was found");
            let h_j = &hailstones.get(p.1).expect("no valid solution was found");
            a_rows.push((h_i.velocity - h_j.velocity).cross(&(h_i.position - h_j.position)));
            b_terms.push((h_i.velocity - h_j.velocity).dot(&(h_i.position.cross(&h_j.position))));
        }

        let a = Matrix3::from_columns(&a_rows).transpose();
        let b = Vector3::from_column_slice(&b_terms);

        if let Some(p) = a.lu().solve(&b) {
            let rounded = p.map(|x| x.round());
            if p.iter()
                .zip(rounded.iter())
                .all(|(x, r)| (x - r).abs() < 0.1)
            {
                return rounded.iter().map(|x| *x as u64).sum();
            }
        }

        pairs.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 47);
    }
}

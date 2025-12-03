use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Position,
}

impl Robot {
    fn move_n(&mut self, n: i32, boundary: &Position) {
        self.position.x =
            ((self.position.x + n * self.velocity.x) % boundary.x + boundary.x) % boundary.x;
        self.position.y =
            ((self.position.y + n * self.velocity.y) % boundary.y + boundary.y) % boundary.y;
    }
}

fn read_position(position: &str) -> Position {
    let (mut x_str, y_str) = position.split_once(',').unwrap();
    x_str = &x_str[2..];
    let x = x_str.parse::<i32>().unwrap();
    let y = y_str.parse::<i32>().unwrap();
    Position { x, y }
}

fn read_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            Robot {
                position: read_position(p),
                velocity: read_position(v),
            }
        })
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], boundary: &Position) -> u64 {
    let center = Position {
        x: boundary.x / 2,
        y: boundary.y / 2,
    };
    robots
        .iter()
        .fold([0, 0, 0, 0], |mut acc, robot| {
            if robot.position.x < center.x && robot.position.y < center.y {
                acc[0] += 1;
            } else if robot.position.x > center.x && robot.position.y < center.y {
                acc[1] += 1;
            } else if robot.position.x < center.x && robot.position.y > center.y {
                acc[2] += 1;
            } else if robot.position.x > center.x && robot.position.y > center.y {
                acc[3] += 1;
            }
            acc
        })
        .iter()
        .product()
}

fn solve_for_boundary(input: &str, boundary: &Position) -> u64 {
    let mut robots = read_robots(&input);
    robots
        .iter_mut()
        .for_each(|robot| robot.move_n(100, &boundary));

    calculate_safety_factor(&robots, &boundary)
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let boundary = Position { x: 101, y: 103 };
    let safety_factor = solve_for_boundary(&input, &boundary);
    format!("The safety factor after 100 seconds is: {}", safety_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let boundary = Position { x: 11, y: 7 };
        let actual_solution = solve_for_boundary(&input, &boundary);
        let expected_solution = 12;
        assert_eq!(actual_solution, expected_solution);
    }
}

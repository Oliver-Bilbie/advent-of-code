use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone, Debug)]
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

fn read_robots(input_file: &str) -> Vec<Robot> {
    fs::read_to_string(input_file)
        .unwrap()
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

fn has_overlap(robots: &[Robot], _boundary: &Position) -> bool {
    let mut position_counts = HashMap::new();

    for robot in robots {
        *position_counts.entry(&robot.position).or_insert(0) += 1;
    }

    position_counts.values().any(|&count| count > 1)
}

fn print_robots(robots: &[Robot], boundary: &Position) {
    (0..boundary.x).for_each(|x| {
        let row: String = (0..boundary.y)
            .map(|y| {
                let search_position = Position { x, y };
                let robots_found = robots
                    .iter()
                    .filter(|robot| robot.position == search_position)
                    .count();
                if robots_found == 0 {
                    "  ".to_string()
                } else {
                    '🤖'.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", row);
    });
}

fn main() {
    let mut robots = read_robots("../input.txt");
    let boundary = Position { x: 101, y: 103 };

    let mut time = 0;

    loop {
        if !has_overlap(&robots, &boundary) {
            break;
        }

        robots
            .iter_mut()
            .for_each(|robot| robot.move_n(1, &boundary));
        time += 1;
    }

    print_robots(&robots, &boundary);
    println!();
    println!("Easter egg found after {} iterations", time);
}

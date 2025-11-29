use rayon::prelude::*;
use wasm_bindgen::prelude::*;

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
    fn move_n(&self, n: i32, boundary: &Position) -> Robot {
        let x = ((self.position.x + n * self.velocity.x) % boundary.x + boundary.x) % boundary.x;
        let y = ((self.position.y + n * self.velocity.y) % boundary.y + boundary.y) % boundary.y;
        Robot {
            position: Position { x, y },
            velocity: self.velocity.clone(),
        }
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

fn calculate_standard_deviation(data: &[u64]) -> f64 {
    let data_iter = data.iter().map(|x| *x as f64);

    let n = data_iter.clone().count() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mean = data_iter.clone().sum::<f64>() / n;
    let variance = data_iter.map(|x: f64| (x - mean).powi(2)).sum::<f64>() / n;

    variance.sqrt()
}

fn move_all_robots(robots: &[Robot], time: usize, boundary: &Position) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| robot.move_n(time as i32, &boundary))
        .collect::<Vec<Robot>>()
}

fn find_anomalies(safety_scores: &[u64], tolerance: f64) -> Vec<usize> {
    let mean = safety_scores.iter().sum::<u64>() as f64 / safety_scores.len() as f64;
    let std_dev = calculate_standard_deviation(&safety_scores);

    safety_scores
        .iter()
        .enumerate()
        .filter_map(|(t, safety_score)| {
            if (*safety_score as f64 - mean).abs() > tolerance * std_dev {
                Some(t)
            } else {
                None
            }
        })
        .collect()
}

fn display_robots(robots: &[Robot], boundary: &Position) -> String {
    let mut robots_string = String::new();
    (0..boundary.y).for_each(|y| {
        let row: String = (0..boundary.x)
            .map(|x| {
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
        robots_string.push_str(&row);
        robots_string.push('\n');
    });
    robots_string
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let robots = read_robots(&input);
    let boundary = Position { x: 101, y: 103 };

    let tolerance = 9.5; // Number of standard deviations from the mean

    let mut safety_scores: Vec<u64> = (0..1000)
        .into_par_iter()
        .map(|t| calculate_safety_factor(&move_all_robots(&robots, t, &boundary), &boundary))
        .collect();
    let mut anomalies = find_anomalies(&safety_scores, tolerance);

    while anomalies.len() == 0 {
        if safety_scores.len() > 100000 {
            return format!("The easter egg was not found! Try reducing the tolerance value.");
        }

        let mut next_safety_scores: Vec<u64> = (safety_scores.len()..safety_scores.len() + 1000)
            .into_par_iter()
            .map(|t| calculate_safety_factor(&move_all_robots(&robots, t, &boundary), &boundary))
            .collect();

        safety_scores.append(&mut next_safety_scores);

        anomalies = find_anomalies(&safety_scores, tolerance);
    }

    let t = anomalies.first().unwrap();
    let tree_formation = robots
        .iter()
        .map(|robot| robot.move_n(*t as i32, &boundary))
        .collect::<Vec<Robot>>();

    let robot_display = display_robots(&tree_formation, &boundary);
    format!(
        "Easter egg found after {} seconds\n(If the image displayed is not a christmas tree, try reducing the font size)\n\n{}",
        t, robot_display
    )
}

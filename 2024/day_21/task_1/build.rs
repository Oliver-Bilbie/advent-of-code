use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use aoc_utils::direction::Direction;
use aoc_utils::position::Position;

fn main() {
    let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("keypad_paths.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    let dir_paths = directional_paths();
    let num_paths = numeric_paths();

    write_array_to_file("NUMERIC_PATHS", num_paths, &mut f);
    write_array_to_file("DIRECTIONAL_PATHS", dir_paths, &mut f);
}

fn directional_paths() -> Vec<Vec<String>> {
    find_best_paths(&directional_positions(), directional_oob())
}

fn numeric_paths() -> Vec<Vec<String>> {
    find_best_paths(&numeric_positions(), numeric_oob())
}

fn find_best_paths(positions: &[Position<i8>], oob: Position<i8>) -> Vec<Vec<String>> {
    let mut dir_paths = vec![vec![String::new(); positions.len()]; positions.len()];

    for start in 0..positions.len() {
        for end in 0..positions.len() {
            let offset = positions[end].clone() - positions[start].clone();
            let mut path = String::new();

            // Optimal paths will have like characters grouped together.
            // Ideally the directions should appear in this order:
            if offset.column < 0 {
                path.push_str(&"<".repeat(-offset.column as usize));
            }
            if offset.row < 0 {
                path.push_str(&"v".repeat(-offset.row as usize));
            }
            if offset.row > 0 {
                path.push_str(&"^".repeat(offset.row as usize));
            }
            if offset.column > 0 {
                path.push_str(&">".repeat(offset.column as usize));
            }

            // Now we check whether the best route is valid.
            // If not, reversing it will keep like characters together
            // while avoiding the out-of-bounds location.
            if !is_valid_path(&path, &positions[start], &oob) {
                path = path.chars().rev().collect();
            }

            path.push('A');
            dir_paths[start][end] = path;
        }
    }

    return dir_paths;
}

fn is_valid_path(path: &String, start: &Position<i8>, oob: &Position<i8>) -> bool {
    let mut position = start.clone();

    for direction in path.chars() {
        position = match direction {
            '^' => Direction::Down.travel(&position),
            'v' => Direction::Up.travel(&position),
            '<' => Direction::Left.travel(&position),
            '>' => Direction::Right.travel(&position),
            _ => panic!("{} is not a valid direction", direction),
        };

        if position == *oob {
            return false;
        }
    }

    true
}

const fn directional_positions() -> [Position<i8>; 5] {
    [
        Position { row: 1, column: 2 },
        Position { row: 1, column: 1 },
        Position { row: 0, column: 0 },
        Position { row: 0, column: 1 },
        Position { row: 0, column: 2 },
    ]
}

const fn directional_oob() -> Position<i8> {
    Position { row: 1, column: 0 }
}

const fn numeric_positions() -> [Position<i8>; 11] {
    [
        Position { row: 0, column: 1 },
        Position { row: 1, column: 0 },
        Position { row: 1, column: 1 },
        Position { row: 1, column: 2 },
        Position { row: 2, column: 0 },
        Position { row: 2, column: 1 },
        Position { row: 2, column: 2 },
        Position { row: 3, column: 0 },
        Position { row: 3, column: 1 },
        Position { row: 3, column: 2 },
        Position { row: 0, column: 2 },
    ]
}

const fn numeric_oob() -> Position<i8> {
    Position { row: 0, column: 0 }
}

fn write_array_to_file(name: &str, data: Vec<Vec<String>>, file_writer: &mut BufWriter<File>) {
    writeln!(
        file_writer,
        "pub const {}: [[&'static str; {}]; {}] = [",
        name,
        data.len(),
        data.len()
    )
    .unwrap();
    for row in data {
        writeln!(file_writer, "    [").unwrap();
        for item in row {
            writeln!(file_writer, "        \"{}\",", item).unwrap();
        }
        writeln!(file_writer, "    ],").unwrap();
    }
    writeln!(file_writer, "];").unwrap();
}

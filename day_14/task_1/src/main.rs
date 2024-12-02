use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Rock {
    Round,
    Cubic,
    None,
}

fn tilt_north(platform: &mut Vec<Vec<Rock>>) {
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for row in 1..platform.len() {
            for column in 0..platform[0].len() {
                if platform[row][column] == Rock::Round {
                    if platform[row - 1][column] == Rock::None {
                        platform[row][column] = Rock::None;
                        platform[row - 1][column] = Rock::Round;
                        something_moved = true;
                    }
                }
            }
        }
    }
}

fn calculate_load_on_north(platform: &Vec<Vec<Rock>>) -> u32 {
    let mut load: u32 = 0;
    for row in 0..platform.len() {
        let row_multiplier = platform.len() - row;
        let round_rock_count = platform[row].iter().filter(|&x| *x == Rock::Round).count();
        load += (round_rock_count as u32) * (row_multiplier as u32);
    }
    load
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut platform: Vec<Vec<Rock>> = Vec::new();

    for line in reader.lines() {
        let mut row: Vec<Rock> = Vec::new();
        for c in line.unwrap().chars() {
            match c {
                '.' => row.push(Rock::None),
                '#' => row.push(Rock::Cubic),
                'O' => row.push(Rock::Round),
                _ => panic!("Invalid rock type"),
            }
        }
        platform.push(row);
    }

    tilt_north(&mut platform);

    let load = calculate_load_on_north(&platform);
    println!("Load on north: {}", load);
}

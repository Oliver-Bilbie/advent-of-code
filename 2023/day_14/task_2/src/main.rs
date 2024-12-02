use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
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

fn tilt_south(platform: &mut Vec<Vec<Rock>>) {
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for row in (0..platform.len() - 1).rev() {
            for column in 0..platform[0].len() {
                if platform[row][column] == Rock::Round {
                    if platform[row + 1][column] == Rock::None {
                        platform[row][column] = Rock::None;
                        platform[row + 1][column] = Rock::Round;
                        something_moved = true;
                    }
                }
            }
        }
    }
}

fn tilt_west(platform: &mut Vec<Vec<Rock>>) {
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for row in 0..platform.len() {
            for column in 1..platform[0].len() {
                if platform[row][column] == Rock::Round {
                    if platform[row][column - 1] == Rock::None {
                        platform[row][column] = Rock::None;
                        platform[row][column - 1] = Rock::Round;
                        something_moved = true;
                    }
                }
            }
        }
    }
}

fn tilt_east(platform: &mut Vec<Vec<Rock>>) {
    let mut something_moved = true;
    while something_moved {
        something_moved = false;
        for row in 0..platform.len() {
            for column in (0..platform[0].len() - 1).rev() {
                if platform[row][column] == Rock::Round {
                    if platform[row][column + 1] == Rock::None {
                        platform[row][column] = Rock::None;
                        platform[row][column + 1] = Rock::Round;
                        something_moved = true;
                    }
                }
            }
        }
    }
}

fn spin_cycle(platform: &mut Vec<Vec<Rock>>) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
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

    let mut seen_platforms: HashMap<Vec<Vec<Rock>>, u32> = HashMap::new();
    let mut cycle_count = 0;
    loop {
        spin_cycle(&mut platform);
        cycle_count += 1;
        if seen_platforms.contains_key(&platform) {
            break;
        }
        seen_platforms.insert(platform.clone(), cycle_count);
    }

    const REQUIRED_CYCLES: u32 = 1000000000;
    let cycles_before_repeat = cycle_count;
    let repeat_length = cycle_count - seen_platforms.get(&platform).unwrap();

    let remaining_cycles = (REQUIRED_CYCLES - cycles_before_repeat) % repeat_length;
    for _ in 0..remaining_cycles {
        spin_cycle(&mut platform);
    }

    let load = calculate_load_on_north(&platform);
    println!("Load on north: {}", load);
}

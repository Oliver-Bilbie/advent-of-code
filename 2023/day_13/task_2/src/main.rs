use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_for_power_of_two(x: u32) -> bool {
    return x != 0 && (x & (x - 1)) == 0;
}

fn check_for_smudged_reflection(hash_1: u32, hash_2: u32) -> bool {
    let xor = hash_1 ^ hash_2;
    if check_for_power_of_two(xor) {
        return true;
    }
    false
}

fn calculate_row_hashes(rock_locations: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut hashes: Vec<u32> = Vec::new();
    for row in rock_locations {
        let mut hash: u32 = 0;
        for rock in row {
            hash <<= 1;
            if *rock {
                hash += 1;
            }
        }
        hashes.push(hash);
    }
    hashes
}

fn calculate_column_hashes(rock_locations: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut hashes: Vec<u32> = Vec::new();
    for i in 0..rock_locations[0].len() {
        let mut hash: u32 = 0;
        for j in 0..rock_locations.len() {
            hash <<= 1;
            if rock_locations[j][i] {
                hash += 1;
            }
        }
        hashes.push(hash);
    }
    hashes
}

fn calculate_pattern_value(pattern: &Vec<Vec<bool>>) -> u16 {
    // This function really needs some refactoring but writing production-grade code
    // is not my goal for advent of code. If you are reading this - sorry
    let row_hashes = calculate_row_hashes(pattern);
    let mut mirror_row: Option<u32> = None;
    let mut smudge_found: bool;

    for i in 0..row_hashes.len() - 1 {
        let mut is_true_reflection = row_hashes[i] == row_hashes[i + 1];
        let mut is_smudged_reflection =
            check_for_smudged_reflection(row_hashes[i], row_hashes[i + 1]);
        smudge_found = is_smudged_reflection.clone();
        if is_true_reflection || is_smudged_reflection {
            mirror_row = Some(i as u32);
            let mut offset = 1;
            while i + 1 + offset < row_hashes.len() && i >= offset {
                is_true_reflection = row_hashes[i - offset] == row_hashes[i + 1 + offset];
                is_smudged_reflection = check_for_smudged_reflection(
                    row_hashes[i - offset],
                    row_hashes[i + 1 + offset],
                );
                if is_smudged_reflection {
                    if smudge_found {
                        mirror_row = None;
                        break;
                    } else {
                        smudge_found = true;
                    }
                }
                if is_true_reflection || is_smudged_reflection {
                    offset += 1;
                } else {
                    mirror_row = None;
                    break;
                }
            }
            if mirror_row.is_some() && smudge_found {
                break;
            } else {
                mirror_row = None;
            }
        }
    }

    match mirror_row {
        Some(row) => {
            return ((row + 1) * 100).try_into().unwrap();
        }
        None => {
            let column_hashes = calculate_column_hashes(pattern);
            let mut mirror_column: Option<u32> = None;
            let mut smudge_found: bool;

            for i in 0..column_hashes.len() - 1 {
                let mut is_true_reflection = column_hashes[i] == column_hashes[i + 1];
                let mut is_smudged_reflection =
                    check_for_smudged_reflection(column_hashes[i], column_hashes[i + 1]);
                smudge_found = is_smudged_reflection.clone();
                if is_true_reflection || is_smudged_reflection {
                    mirror_column = Some(i as u32);
                    let mut offset = 1;
                    while i + 1 + offset < column_hashes.len() && i >= offset {
                        is_true_reflection =
                            column_hashes[i - offset] == column_hashes[i + 1 + offset];
                        is_smudged_reflection = check_for_smudged_reflection(
                            column_hashes[i - offset],
                            column_hashes[i + 1 + offset],
                        );
                        if is_smudged_reflection {
                            if smudge_found {
                                mirror_column = None;
                                break;
                            } else {
                                smudge_found = true;
                            }
                        }
                        if is_true_reflection || is_smudged_reflection {
                            offset += 1;
                        } else {
                            mirror_column = None;
                            break;
                        }
                    }
                    if mirror_column.is_some() && smudge_found {
                        break;
                    } else {
                        mirror_column = None;
                    }
                }
            }

            match mirror_column {
                Some(column) => {
                    return (column + 1).try_into().unwrap();
                }
                None => {
                    panic!("No mirror found");
                }
            }
        }
    }
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_value: u64 = 0;
    let mut rock_locations: Vec<Vec<bool>> = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        if row.len() == 0 {
            total_value += calculate_pattern_value(&rock_locations) as u64;
            rock_locations = Vec::new();
        } else {
            let rock_row: Vec<bool> = row.chars().map(|c| c == '#').collect();
            rock_locations.push(rock_row);
        }
    }
    total_value += calculate_pattern_value(&rock_locations) as u64;

    println!("Total value: {}", total_value);
}

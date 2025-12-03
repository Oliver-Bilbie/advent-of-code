use wasm_bindgen::prelude::*;

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
    let row_hashes = calculate_row_hashes(pattern);
    let mut mirror_row: Option<u32> = None;
    for i in 0..row_hashes.len() - 1 {
        if row_hashes[i] == row_hashes[i + 1] {
            mirror_row = Some(i as u32);
            let mut offset = 1;
            while i + 1 + offset < row_hashes.len() && i >= offset {
                if row_hashes[i - offset] == row_hashes[i + 1 + offset] {
                    offset += 1;
                } else {
                    mirror_row = None;
                    break;
                }
            }
            if mirror_row.is_some() {
                break;
            }
        }
    }

    match mirror_row {
        Some(row) => {
            println!("Found mirror row at {}", row);
            return ((row + 1) * 100).try_into().unwrap();
        }
        None => {
            let column_hashes = calculate_column_hashes(pattern);
            let mut mirror_column: Option<u32> = None;
            for i in 0..column_hashes.len() - 1 {
                if column_hashes[i] == column_hashes[i + 1] {
                    mirror_column = Some(i as u32);
                    let mut offset = 1;
                    while i + 1 + offset < column_hashes.len() && i >= offset {
                        if column_hashes[i - offset] == column_hashes[i + 1 + offset] {
                            offset += 1;
                        } else {
                            mirror_column = None;
                            break;
                        }
                    }
                    if mirror_column.is_some() {
                        break;
                    }
                }
            }

            match mirror_column {
                Some(column) => {
                    println!("Found mirror column at {}", column);
                    return (column + 1).try_into().unwrap();
                }
                None => {
                    panic!("No mirror found");
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut total_value: u64 = 0;
    let mut rock_locations: Vec<Vec<bool>> = Vec::new();

    for row in input.lines() {
        if row.len() == 0 {
            total_value += calculate_pattern_value(&rock_locations) as u64;
            rock_locations = Vec::new();
        } else {
            let rock_row: Vec<bool> = row.chars().map(|c| c == '#').collect();
            rock_locations.push(rock_row);
        }
    }
    total_value += calculate_pattern_value(&rock_locations) as u64;

    format!("Total value: {}", total_value)
}

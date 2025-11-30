use wasm_bindgen::prelude::*;

struct Galaxy {
    x: u16,
    y: u16,
}

fn read_data(input: &str) -> Vec<Vec<bool>> {
    let mut data: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut line_data: Vec<bool> = Vec::new();
        let entries: Vec<char> = line.chars().collect();

        for entry in entries {
            match entry {
                '.' => line_data.push(false),
                '#' => line_data.push(true),
                _ => panic!("Invalid character"),
            }
        }

        data.push(line_data);
    }

    data
}

fn find_expanded_rows(data: &Vec<Vec<bool>>) -> Vec<u16> {
    let mut expanded_rows: Vec<u16> = Vec::new();

    for y in 0..data.len() {
        let mut galaxy_in_row = false;
        for x in 0..data[0].len() {
            if data[y][x] {
                galaxy_in_row = true;
                break;
            }
        }
        if !galaxy_in_row {
            expanded_rows.push(y.try_into().unwrap());
        }
    }

    expanded_rows
}

fn find_expanded_columns(data: &Vec<Vec<bool>>) -> Vec<u16> {
    let mut expanded_columns: Vec<u16> = Vec::new();

    for x in 0..data[0].len() {
        let mut galaxy_in_column = false;
        for y in 0..data.len() {
            if data[y][x] {
                galaxy_in_column = true;
                break;
            }
        }
        if !galaxy_in_column {
            expanded_columns.push(x.try_into().unwrap());
        }
    }

    expanded_columns
}

fn find_galaxies(data: &Vec<Vec<bool>>) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] {
                galaxies.push(Galaxy {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
    }

    galaxies
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let expansion_factor: u32 = 1000000;

    let data = read_data(input);
    let expanded_rows = find_expanded_rows(&data);
    let expanded_columns = find_expanded_columns(&data);
    let galaxies = find_galaxies(&data);

    let mut distances: Vec<u64> = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            // Convert to i32 for ease of calculating absolute differences
            let galaxy_a_x: i32 = galaxies[i].x.into();
            let galaxy_a_y: i32 = galaxies[i].y.into();
            let galaxy_b_x: i32 = galaxies[j].x.into();
            let galaxy_b_y: i32 = galaxies[j].y.into();

            let raw_distance: u32 = ((galaxy_a_x - galaxy_b_x).abs()
                + (galaxy_a_y - galaxy_b_y).abs())
            .try_into()
            .unwrap();

            let mut x_expansion_distance: u32 = 0;
            for column in expanded_columns.clone() {
                let is_between = (galaxies[i].x < column && galaxies[j].x > column)
                    || (galaxies[i].x > column && galaxies[j].x < column);
                if is_between {
                    x_expansion_distance += expansion_factor - 1;
                }
            }

            let mut y_expansion_distance: u32 = 0;
            for row in expanded_rows.clone() {
                let is_between = (galaxies[i].y < row && galaxies[j].y > row)
                    || (galaxies[i].y > row && galaxies[j].y < row);
                if is_between {
                    y_expansion_distance += expansion_factor - 1;
                }
            }

            let distance: u64 = (raw_distance + x_expansion_distance + y_expansion_distance).into();
            distances.push(distance);
        }
    }

    let sum_of_distances: u64 = distances.iter().sum();

    format!("Sum of distances: {}", sum_of_distances)
}

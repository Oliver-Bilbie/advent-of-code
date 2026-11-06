use aoc_utils::{direction::Direction, grid::Grid, position::Position};
use wasm_bindgen::prelude::*;

type MovePair = (Option<Direction>, Option<Direction>);

const ENHANCE_COUNT: u16 = 2;
const OFFSETS: [MovePair; 9] = [
    (Some(Direction::Up), Some(Direction::Left)),
    (Some(Direction::Up), None),
    (Some(Direction::Up), Some(Direction::Right)),
    (None, Some(Direction::Left)),
    (None, None),
    (None, Some(Direction::Right)),
    (Some(Direction::Down), Some(Direction::Left)),
    (Some(Direction::Down), None),
    (Some(Direction::Down), Some(Direction::Right)),
];

fn read_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid character: '{}' (must be '#' or '.')", c),
    }
}

fn read_algorithm(algorithm_input: &str) -> Vec<bool> {
    algorithm_input.chars().map(|c| read_char(c)).collect()
}

fn read_image(image_input: &str) -> Grid<bool> {
    let mapper = |line: &str| line.chars().map(|c| read_char(c)).collect();
    Grid::from_str(image_input, mapper)
}

fn get_neighbor(image: &Grid<bool>, position: &Position, offset: &MovePair, is_lit: bool) -> usize {
    let void_value = if is_lit { 1 } else { 0 };
    let mut neighbor_pos = position.clone();
    if let Some(row_offset) = &offset.0 {
        neighbor_pos = match row_offset.travel_with_bounds(&neighbor_pos, image.size()) {
            Some(p) => p,
            None => return void_value,
        }
    }
    if let Some(col_offset) = &offset.1 {
        neighbor_pos = match col_offset.travel_with_bounds(&neighbor_pos, image.size()) {
            Some(p) => p,
            None => return void_value,
        }
    }
    if *image.get(&neighbor_pos).unwrap() {
        1
    } else {
        0
    }
}

fn pad(image: &Grid<bool>, is_lit: bool) -> Grid<bool> {
    let size = image.size() + Position { row: 2, column: 2 };
    let mut data = Vec::with_capacity(size.row * size.column);
    let mut img_iter = image.iter();

    for _ in 0..size.column {
        data.push(is_lit);
    }
    for _ in 0..image.size().row {
        data.push(is_lit);
        for _ in 0..image.size().column {
            data.push(img_iter.next().unwrap().clone());
        }
        data.push(is_lit);
    }
    for _ in 0..size.column {
        data.push(is_lit);
    }

    Grid::from_vec(data, size)
}

fn enhance(image: &Grid<bool>, algorithm: &Vec<bool>, void_is_lit: bool) -> Grid<bool> {
    let img = pad(image, void_is_lit);
    let mut result = img.clone();

    for row in 0..img.size().row {
        for column in 0..img.size().column {
            let pos = Position { row, column };
            let id = OFFSETS.iter().fold(0, |acc, offset| {
                acc * 2 + get_neighbor(&img, &pos, offset, void_is_lit)
            });
            result.set(&pos, algorithm[id]);
        }
    }

    result
}

fn result(input: &str) -> u64 {
    let (algo_input, img_input) = input.split_once("\n\n").expect("an image must be provided");
    let algorithm = read_algorithm(algo_input);
    let mut image = read_image(img_input);
    let mut void_is_lit = false;

    for _ in 0..ENHANCE_COUNT {
        image = enhance(&image, &algorithm, void_is_lit);
        // For the real input the void flashes on/off, so we must track and account for this
        if void_is_lit {
            void_is_lit = algorithm[511];
        } else {
            void_is_lit = algorithm[0];
        }
    }

    image.iter().filter(|&&x| x).count() as u64
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!(
        "There are {} lit pixels after enhancing twice",
        result(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 35);
    }
}

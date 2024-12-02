use std::fs;

enum TileContent {
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
    Empty,
}

impl TileContent {
    fn from_char(tile_char: char) -> TileContent {
        match tile_char {
            '/' => TileContent::ForwardMirror,
            '\\' => TileContent::BackwardMirror,
            '|' => TileContent::VerticalSplitter,
            '-' => TileContent::HorizontalSplitter,
            '.' => TileContent::Empty,
            _ => panic!("Invalid tile character"),
        }
    }
}

struct Tile {
    content: TileContent,
    history: Vec<Beam>,
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(PartialEq, Clone)]
struct Beam {
    position: Position,
    direction: Direction,
    is_init: bool,
}

fn read_tiles(source_file: &str) -> Vec<Vec<Tile>> {
    let input_string = fs::read_to_string(source_file).unwrap();
    let mut tiles: Vec<Vec<Tile>> = vec![];

    for input_line in input_string.lines() {
        let mut tiles_row: Vec<Tile> = vec![];
        for tile_char in input_line.chars() {
            let new_tile = Tile {
                content: TileContent::from_char(tile_char),
                history: vec![],
            };
            tiles_row.push(new_tile);
        }
        tiles.push(tiles_row);
    }
    tiles
}

fn reset_tiles(tiles: &mut Vec<Vec<Tile>>) {
    for row in tiles {
        for tile in row {
            tile.history = vec![];
        }
    }
}

fn get_grid_size(tiles: &Vec<Vec<Tile>>) -> Position {
    let row = tiles.len();
    let column = tiles.first().unwrap().len();

    Position { row, column }
}

fn update_beams(beams: &mut Vec<Beam>, tiles: &mut Vec<Vec<Tile>>) {
    let mut updated_beams: Vec<Beam> = vec![];
    for beam in &mut *beams {
        let is_duplicate = tiles[beam.position.row][beam.position.column]
            .history
            .contains(&beam);

        if !is_duplicate {
            tiles[beam.position.row][beam.position.column]
                .history
                .push(beam.clone());

            let mut new_beam_states = evaluate_next_beam_state(beam.clone(), &tiles);
            updated_beams.append(&mut new_beam_states);
        }
    }
    *beams = updated_beams;
}

fn evaluate_next_beam_position(beam: &Beam, max_row: usize, max_col: usize) -> Option<Position> {
    if beam.is_init {
        return Some(beam.position.clone());
    }

    let mut position: Option<Position> = None;
    match beam.direction {
        Direction::Up => {
            if beam.position.row > 0 {
                position = Some(Position {
                    row: beam.position.row - 1,
                    column: beam.position.column,
                })
            }
        }
        Direction::Down => {
            if beam.position.row < max_row - 1 {
                position = Some(Position {
                    row: beam.position.row + 1,
                    column: beam.position.column,
                })
            }
        }
        Direction::Left => {
            if beam.position.column > 0 {
                position = Some(Position {
                    row: beam.position.row,
                    column: beam.position.column - 1,
                })
            }
        }
        Direction::Right => {
            if beam.position.column < max_col - 1 {
                position = Some(Position {
                    row: beam.position.row,
                    column: beam.position.column + 1,
                })
            }
        }
    };
    position
}

fn evaluate_next_beam_state(beam: Beam, tiles: &Vec<Vec<Tile>>) -> Vec<Beam> {
    let grid_size = get_grid_size(tiles);

    let next_position: Position =
        match evaluate_next_beam_position(&beam, grid_size.row, grid_size.column) {
            Some(value) => value,
            None => return vec![],
        };

    match tiles[next_position.row][next_position.column].content {
        TileContent::Empty => vec![Beam {
            position: next_position,
            direction: beam.direction,
            is_init: false,
        }],
        TileContent::ForwardMirror => match beam.direction {
            Direction::Up => vec![Beam {
                position: next_position,
                direction: Direction::Right,
                is_init: false,
            }],
            Direction::Down => vec![Beam {
                position: next_position,
                direction: Direction::Left,
                is_init: false,
            }],
            Direction::Left => vec![Beam {
                position: next_position,
                direction: Direction::Down,
                is_init: false,
            }],
            Direction::Right => vec![Beam {
                position: next_position,
                direction: Direction::Up,
                is_init: false,
            }],
        },
        TileContent::BackwardMirror => match beam.direction {
            Direction::Up => vec![Beam {
                position: next_position,
                direction: Direction::Left,
                is_init: false,
            }],
            Direction::Down => vec![Beam {
                position: next_position,
                direction: Direction::Right,
                is_init: false,
            }],
            Direction::Left => vec![Beam {
                position: next_position,
                direction: Direction::Up,
                is_init: false,
            }],
            Direction::Right => vec![Beam {
                position: next_position,
                direction: Direction::Down,
                is_init: false,
            }],
        },
        TileContent::HorizontalSplitter => match beam.direction {
            Direction::Up | Direction::Down => vec![
                Beam {
                    position: next_position.clone(),
                    direction: Direction::Left,
                    is_init: false,
                },
                Beam {
                    position: next_position.clone(),
                    direction: Direction::Right,
                    is_init: false,
                },
            ],
            Direction::Left | Direction::Right => vec![Beam {
                position: next_position,
                direction: beam.direction,
                is_init: false,
            }],
        },
        TileContent::VerticalSplitter => match beam.direction {
            Direction::Left | Direction::Right => vec![
                Beam {
                    position: next_position.clone(),
                    direction: Direction::Up,
                    is_init: false,
                },
                Beam {
                    position: next_position.clone(),
                    direction: Direction::Down,
                    is_init: false,
                },
            ],
            Direction::Up | Direction::Down => vec![Beam {
                position: next_position,
                direction: beam.direction,
                is_init: false,
            }],
        },
    }
}

fn count_energized_tiles(tiles: &Vec<Vec<Tile>>) -> u32 {
    let mut energized_tiles: u32 = 0;
    for row in tiles {
        for column in row {
            if column.history.len() > 0 {
                energized_tiles += 1;
            }
        }
    }
    energized_tiles
}

fn get_initial_position(direction: &Direction, offset: usize, tiles: &Vec<Vec<Tile>>) -> Position {
    let grid_size = get_grid_size(&tiles);

    match direction {
        Direction::Down => Position {
            row: 0,
            column: offset,
        },
        Direction::Up => Position {
            row: grid_size.row - 1,
            column: offset,
        },
        Direction::Right => Position {
            row: offset,
            column: 0,
        },
        Direction::Left => Position {
            row: 0,
            column: grid_size.column - 1,
        },
    }
}

fn evaluate_energization_state(
    direction: Direction,
    offset: usize,
    tiles: &mut Vec<Vec<Tile>>,
) -> u32 {
    let mut beams: Vec<Beam> = vec![Beam {
        position: get_initial_position(&direction, offset, &tiles),
        direction,
        is_init: true,
    }];

    while beams.len() > 0 {
        update_beams(&mut beams, tiles);
    }

    count_energized_tiles(&tiles)
}

fn main() {
    let mut tiles = read_tiles("../input.txt");
    let grid_size = get_grid_size(&tiles);
    let mut max_energized_tiles = 0;

    for direction in [Direction::Right, Direction::Left] {
        for offset in 0..grid_size.row {
            reset_tiles(&mut tiles);
            let energized_tiles =
                evaluate_energization_state(direction.clone(), offset, &mut tiles);
            if energized_tiles > max_energized_tiles {
                max_energized_tiles = energized_tiles;
            }
        }
    }
    for direction in [Direction::Down, Direction::Up] {
        for offset in 0..grid_size.column {
            reset_tiles(&mut tiles);
            let energized_tiles =
                evaluate_energization_state(direction.clone(), offset, &mut tiles);
            if energized_tiles > max_energized_tiles {
                max_energized_tiles = energized_tiles;
            }
        }
    }

    println!(
        "The maximum number of energized tiles is {}",
        max_energized_tiles
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_first_example() {
        let mut tiles = read_tiles("../test_input.txt");
        let mut beams: Vec<Beam> = vec![Beam {
            position: Position { row: 0, column: 0 },
            direction: Direction::Right,
            is_init: true,
        }];
        while beams.len() > 0 {
            update_beams(&mut beams, &mut tiles);
        }
        let actual_tiles = count_energized_tiles(&tiles);
        let expected_tiles = 46;
        assert_eq!(actual_tiles, expected_tiles);
    }

    #[test]
    fn it_resets_all_tiles() {
        let mut tiles = read_tiles("../test_input.txt");
        let mut beams: Vec<Beam> = vec![Beam {
            position: Position { row: 0, column: 0 },
            direction: Direction::Right,
            is_init: true,
        }];
        while beams.len() > 0 {
            update_beams(&mut beams, &mut tiles);
        }
        reset_tiles(&mut tiles);
        let actual_tiles = count_energized_tiles(&tiles);
        let expected_tiles = 0;
        assert_eq!(actual_tiles, expected_tiles);
    }

    #[test]
    fn it_solves_the_second_example() {
        let mut tiles = read_tiles("../test_input.txt");
        let grid_size = get_grid_size(&tiles);
        let mut max_energized_tiles = 0;
        for direction in [Direction::Right, Direction::Left] {
            for offset in 0..grid_size.row {
                reset_tiles(&mut tiles);
                let energized_tiles =
                    evaluate_energization_state(direction.clone(), offset, &mut tiles);
                if energized_tiles > max_energized_tiles {
                    max_energized_tiles = energized_tiles;
                }
            }
        }
        for direction in [Direction::Down, Direction::Up] {
            for offset in 0..grid_size.column {
                reset_tiles(&mut tiles);
                let energized_tiles =
                    evaluate_energization_state(direction.clone(), offset, &mut tiles);
                if energized_tiles > max_energized_tiles {
                    max_energized_tiles = energized_tiles;
                }
            }
        }
        let actual_tiles = max_energized_tiles;
        let expected_tiles = 51;
        assert_eq!(actual_tiles, expected_tiles);
    }
}

use wasm_bindgen::prelude::*;

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

fn read_tiles(input: &str) -> Vec<Vec<Tile>> {
    let mut tiles: Vec<Vec<Tile>> = vec![];

    for input_line in input.lines() {
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
    let max_row = tiles.len();
    let max_col = tiles.first().unwrap().len();

    let next_position: Position = match evaluate_next_beam_position(&beam, max_row, max_col) {
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

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut tiles = read_tiles(input);

    let mut beams: Vec<Beam> = vec![Beam {
        position: Position { row: 0, column: 0 },
        direction: Direction::Right,
        is_init: true,
    }];

    while beams.len() > 0 {
        update_beams(&mut beams, &mut tiles);
    }

    let energized_tiles = count_energized_tiles(&tiles);

    format!("There are {} energized tiles", energized_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
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
}

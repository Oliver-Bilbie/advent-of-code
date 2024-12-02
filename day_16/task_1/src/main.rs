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

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(PartialEq)]
struct Beam {
    position: Position,
    direction: Direction,
}

fn read_tiles() -> Vec<Vec<Tile>> {
    let input_string = fs::read_to_string("../input.txt").unwrap();
    let mut tiles: Vec<Vec<Tile>> = vec![];

    for input_line in input_string[0..input_string.len() - 1].split("\n") {
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

fn update_beams(beams: Vec<Beam>, tiles: &Vec<Vec<Tile>>) -> Vec<Beam> {
    let mut updated_beams: Vec<Beam> = vec![];
    for beam in beams {
        // TODO: Get new direction (or remove if necessary)
    }
}

fn evaluate_next_beam_position(beam: Beam) -> Option<Position> {
    match beam.direction {
        Up => {
            if beam.position.row > 0 {
                Some(Position {
                    row: beam.position.row - 1,
                    column: beam.position.column,
                })
            } else {
                None
            }
        }
        Down => {
            // TODO: Set boundaries dynamically
            if beam.position.row < 10 {
                Some(Position {
                    row: beam.position.row + 1,
                    column: beam.position.column,
                })
            } else {
                None
            }
        }
        Left => {
            if beam.position.column > 0 {
                Some(Position {
                    row: beam.position.row,
                    column: beam.position.column - 1,
                })
            } else {
                None
            }
        }
        Right => {
            // TODO: Set boundaries dynamically
            if beam.position.column < 10 {
                Some(Position {
                    row: beam.position.row,
                    column: beam.position.column + 1,
                })
            } else {
                None
            }
        }
    }
}

fn evaluate_next_beam_state(beam: Beam, tiles: &Vec<Vec<Tile>>) -> Vec<Beam> {
    let next_position: Position;
    match evaluate_next_beam_position(beam) {
        Some(value) => next_position = value,
        None => return vec![],
    }

    match tiles[next_position.row][next_position.column].content {
        TileContent::Empty => vec![Beam {
            position: next_position,
            direction: beam.direction,
        }],
        TileContent::ForwardMirror => match beam.direction {
            Up => vec![Beam {
                position: next_position,
                direction: Direction::Right,
            }],
            Down => vec![Beam {
                position: next_position,
                direction: Direction::Left,
            }],
            Left => vec![Beam {
                position: next_position,
                direction: Direction::Up,
            }],
            Right => vec![Beam {
                position: next_position,
                direction: Direction::Down,
            }],
        },
        TileContent::BackwardMirror => match beam.direction {
            Up => vec![Beam {
                position: next_position,
                direction: Direction::Left,
            }],
            Down => vec![Beam {
                position: next_position,
                direction: Direction::Right,
            }],
            Left => vec![Beam {
                position: next_position,
                direction: Direction::Down,
            }],
            Right => vec![Beam {
                position: next_position,
                direction: Direction::Up,
            }],
        },
        TileContent::HorizontalSplitter => match beam.direction {
            Direction::Up | Direction::Down => vec![
                Beam {
                    position: next_position,
                    direction: Direction::Left,
                },
                Beam {
                    position: next_position,
                    direction: Direction::Right,
                },
            ],
            Direction::Left | Direction::Right => vec![Beam {
                position: next_position,
                direction: beam.direction,
            }],
        },
        TileContent::VerticalSplitter => match beam.direction {
            Direction::Left | Direction::Right => vec![
                Beam {
                    position: next_position,
                    direction: Direction::Up,
                },
                Beam {
                    position: next_position,
                    direction: Direction::Down,
                },
            ],
            Direction::Up | Direction::Down => vec![Beam {
                position: next_position,
                direction: beam.direction,
            }],
        },
    }
}

fn main() {
    let mut tiles = read_tiles();

    let mut beams: Vec<Beam> = vec![Beam {
        position: Position { row: 0, column: 0 },
        direction: Direction::Right,
    }];

    // Update beams until a None is returned
    // Count energized tiles
}

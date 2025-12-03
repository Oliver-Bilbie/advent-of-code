use wasm_bindgen::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn move_position(&self, position: Position) -> Position {
        match self {
            Direction::North => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::South => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::East => Position {
                x: position.x + 1,
                y: position.y,
            },
            Direction::West => Position {
                x: position.x - 1,
                y: position.y,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PipeDirection {
    from: Direction,
    to: Direction,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Segment {
    Start,
    Pipe(PipeDirection),
    Ground,
}

impl Segment {
    fn from_char(c: char) -> Segment {
        match c {
            '|' => Segment::Pipe(PipeDirection {
                from: Direction::North,
                to: Direction::South,
            }),
            '-' => Segment::Pipe(PipeDirection {
                from: Direction::East,
                to: Direction::West,
            }),
            'L' => Segment::Pipe(PipeDirection {
                from: Direction::North,
                to: Direction::East,
            }),
            'J' => Segment::Pipe(PipeDirection {
                from: Direction::North,
                to: Direction::West,
            }),
            '7' => Segment::Pipe(PipeDirection {
                from: Direction::South,
                to: Direction::West,
            }),
            'F' => Segment::Pipe(PipeDirection {
                from: Direction::South,
                to: Direction::East,
            }),
            'S' => Segment::Start,
            '.' => Segment::Ground,
            _ => panic!("Invalid character"),
        }
    }

    fn get_reverse(&self) -> Segment {
        match self {
            Segment::Pipe(pipe_direction) => Segment::Pipe(PipeDirection {
                from: pipe_direction.to.clone(),
                to: pipe_direction.from.clone(),
            }),
            _ => self.clone(),
        }
    }
}

fn find_first_segment(
    start_position: Position,
    mut segments: Vec<Vec<Segment>>,
    reverse: bool,
) -> (Position, Vec<Vec<Segment>>) {
    // Find a suitable segment adjacent to the starting position.
    // If necessary, reverse its direction to fit.
    // Since S is not along any border, we don't need to
    // check for out-of-bounds errors :-)

    struct FirstPipeLocations {
        x_offset: i8,
        y_offset: i8,
        required_direction: Direction,
    }

    let mut first_pipe_locations = [
        FirstPipeLocations {
            x_offset: 0,
            y_offset: -1,
            required_direction: Direction::South,
        },
        FirstPipeLocations {
            x_offset: 0,
            y_offset: 1,
            required_direction: Direction::North,
        },
        FirstPipeLocations {
            x_offset: -1,
            y_offset: 0,
            required_direction: Direction::East,
        },
        FirstPipeLocations {
            x_offset: 1,
            y_offset: 0,
            required_direction: Direction::West,
        },
    ];

    if reverse {
        first_pipe_locations.reverse();
    }

    for search_location in first_pipe_locations {
        let destination_x = start_position.x as i16 + search_location.x_offset as i16;
        let destination_y = start_position.y as i16 + search_location.y_offset as i16;
        let destination_segment = segments[destination_y as usize][destination_x as usize].clone();

        match destination_segment {
            Segment::Pipe(ref destination_pipe_direction) => {
                if destination_pipe_direction.from == search_location.required_direction {
                    return (
                        Position {
                            x: destination_x as u16,
                            y: destination_y as u16,
                        },
                        segments,
                    );
                } else if destination_pipe_direction.to == search_location.required_direction {
                    segments[destination_y as usize][destination_x as usize] =
                        destination_segment.get_reverse();
                    return (
                        Position {
                            x: destination_x as u16,
                            y: destination_y as u16,
                        },
                        segments,
                    );
                }
            }
            _ => {}
        }
    }
    panic!("No first segment found");
}

fn find_next_segment(
    position: Position,
    to_direction: Direction,
    mut segments: Vec<Vec<Segment>>,
) -> (Position, Vec<Vec<Segment>>) {
    let next_position = to_direction.move_position(position);
    let next_segment = segments[next_position.y as usize][next_position.x as usize].clone();
    match next_segment {
        Segment::Pipe(ref next_pipe_direction) => {
            if next_pipe_direction.from != to_direction.get_opposite() {
                segments[next_position.y as usize][next_position.x as usize] =
                    next_segment.get_reverse();
            };
        }
        _ => {
            panic!("Invalid next segment")
        }
    };

    (next_position, segments)
}

fn read_segments(input: &str) -> (Vec<Vec<Segment>>, Position) {
    let mut segments: Vec<Vec<Segment>> = Vec::new();
    let mut start_position: Option<Position> = None;

    for (y, line) in input.lines().enumerate() {
        let mut row_segments: Vec<Segment> = Vec::new();
        for (x, segment) in line.chars().enumerate() {
            let segment = Segment::from_char(segment);
            if segment == Segment::Start {
                start_position = Some(Position {
                    x: x as u16,
                    y: y as u16,
                });
            }
            row_segments.push(segment);
        }
        segments.push(row_segments);
    }
    (segments, start_position.expect("No start position found"))
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut segments: Vec<Vec<Segment>>;
    let start_position: Position;
    (segments, start_position) = read_segments(input);

    let mut current_positions = [start_position.clone(), start_position.clone()];

    let mut steps: u32 = 0;
    let mut end_reached = false;

    while !end_reached {
        let current_segments = [
            segments[current_positions[0].y as usize][current_positions[0].x as usize].clone(),
            segments[current_positions[1].y as usize][current_positions[1].x as usize].clone(),
        ];

        let mut next_positions: [Option<Position>; 2] = [None, None];
        for (i, current_segment) in current_segments.iter().enumerate() {
            let next_position: Position;
            let updated_segments: Vec<Vec<Segment>>;
            match current_segment {
                Segment::Start => {
                    (next_position, updated_segments) =
                        find_first_segment(current_positions[i].clone(), segments, i == 1);
                }

                Segment::Pipe(pipe_direction) => {
                    (next_position, updated_segments) = find_next_segment(
                        current_positions[i].clone(),
                        pipe_direction.to.clone(),
                        segments,
                    );
                }

                Segment::Ground => {
                    panic!("We are no longer in a pipe!");
                }
            }
            segments = updated_segments;
            next_positions[i] = Some(next_position.clone());
        }

        steps += 1;

        if next_positions[0] == next_positions[1] {
            end_reached = true;
        } else {
            for (i, next_position) in next_positions.iter().enumerate() {
                match next_position {
                    Some(next_position) => {
                        current_positions[i] = next_position.clone();
                    }
                    None => {
                        panic!("No next position found");
                    }
                }
            }
        };
    }

    format!("The furthest point is {} steps away", steps)
}

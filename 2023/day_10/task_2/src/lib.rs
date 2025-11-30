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
struct PipeDetails {
    from: Direction,
    to: Direction,
    char: char,
    main_loop: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Segment {
    Start,
    Pipe(PipeDetails),
    Ground,
}

impl Segment {
    fn from_char(c: char) -> Segment {
        match c {
            '|' => Segment::Pipe(PipeDetails {
                from: Direction::North,
                to: Direction::South,
                char: c,
                main_loop: false,
            }),
            '-' => Segment::Pipe(PipeDetails {
                from: Direction::East,
                to: Direction::West,
                char: c,
                main_loop: false,
            }),
            'L' => Segment::Pipe(PipeDetails {
                from: Direction::North,
                to: Direction::East,
                char: c,
                main_loop: false,
            }),
            'J' => Segment::Pipe(PipeDetails {
                from: Direction::North,
                to: Direction::West,
                char: c,
                main_loop: false,
            }),
            '7' => Segment::Pipe(PipeDetails {
                from: Direction::South,
                to: Direction::West,
                char: c,
                main_loop: false,
            }),
            'F' => Segment::Pipe(PipeDetails {
                from: Direction::South,
                to: Direction::East,
                char: c,
                main_loop: false,
            }),
            'S' => Segment::Start,
            '.' => Segment::Ground,
            _ => panic!("Invalid character"),
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
            Segment::Pipe(ref destination_pipe_details) => {
                if destination_pipe_details.from == search_location.required_direction {
                    segments[destination_y as usize][destination_x as usize] =
                        Segment::Pipe(PipeDetails {
                            from: destination_pipe_details.from.clone(),
                            to: destination_pipe_details.to.clone(),
                            char: destination_pipe_details.char,
                            main_loop: true,
                        });
                    return (
                        Position {
                            x: destination_x as u16,
                            y: destination_y as u16,
                        },
                        segments,
                    );
                } else if destination_pipe_details.to == search_location.required_direction {
                    segments[destination_y as usize][destination_x as usize] =
                        Segment::Pipe(PipeDetails {
                            from: destination_pipe_details.to.clone(),
                            to: destination_pipe_details.from.clone(),
                            char: destination_pipe_details.char,
                            main_loop: true,
                        });
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
                    Segment::Pipe(PipeDetails {
                        from: next_pipe_direction.to.clone(),
                        to: next_pipe_direction.from.clone(),
                        char: next_pipe_direction.char,
                        main_loop: true,
                    });
            } else {
                segments[next_position.y as usize][next_position.x as usize] =
                    Segment::Pipe(PipeDetails {
                        from: next_pipe_direction.from.clone(),
                        to: next_pipe_direction.to.clone(),
                        char: next_pipe_direction.char,
                        main_loop: true,
                    });
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

fn count_segments_inside_loop(segments: Vec<Vec<Segment>>) -> u64 {
    let mut count: u64 = 0;
    for row in segments {
        let mut blocks_to_west = 0;
        let mut section_opened_with: Option<char> = None;

        for segment in &row {
            match segment {
                Segment::Pipe(pipe_details) => match pipe_details.main_loop {
                    true => match pipe_details.char {
                        '|' => {
                            blocks_to_west += 1;
                        }
                        'L' | 'F' => {
                            section_opened_with = Some(pipe_details.char);
                        }
                        'J' => {
                            if section_opened_with.expect("A section is not open") == 'F' {
                                blocks_to_west += 1;
                            }
                            section_opened_with = None;
                        }
                        '7' => {
                            if section_opened_with.expect("A section is not open") == 'L' {
                                blocks_to_west += 1;
                            }
                            section_opened_with = None;
                        }
                        _ => {}
                    },
                    false => {
                        if blocks_to_west % 2 == 1 {
                            count += 1;
                        } else {
                        }
                    }
                },
                Segment::Ground => {
                    if blocks_to_west % 2 == 1 {
                        count += 1;
                    }
                }
                Segment::Start => {}
            }
        }
    }
    count
}

fn find_s_equivalent_segment(segment_1: Segment, segment_2: Segment) -> Segment {
    let from_direction = match segment_1 {
        Segment::Pipe(pipe_details) => pipe_details.from.get_opposite(),
        _ => panic!("Invalid segment"),
    };
    let to_direction = match segment_2 {
        Segment::Pipe(pipe_details) => pipe_details.from.get_opposite(),
        _ => panic!("Invalid segment"),
    };

    let s_char: char;

    match (from_direction.clone(), to_direction.clone()) {
        (Direction::North, Direction::South) | (Direction::South, Direction::North) => s_char = '|',
        (Direction::East, Direction::West) | (Direction::West, Direction::East) => s_char = '-',
        (Direction::North, Direction::East) => s_char = 'L',
        (Direction::North, Direction::West) => s_char = 'J',
        (Direction::South, Direction::West) => s_char = '7',
        (Direction::South, Direction::East) => s_char = 'F',
        _ => panic!("Invalid directions"),
    };

    Segment::Pipe(PipeDetails {
        from: from_direction,
        to: to_direction,
        char: s_char,
        main_loop: true,
    })
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let mut segments: Vec<Vec<Segment>>;
    let start_position: Position;
    (segments, start_position) = read_segments(input);

    let mut current_positions = [start_position.clone(), start_position.clone()];
    (current_positions[0], segments) =
        find_first_segment(current_positions[0].clone(), segments, false);
    (current_positions[1], segments) =
        find_first_segment(current_positions[1].clone(), segments, true);
    let start_segment_as_pipe = find_s_equivalent_segment(
        segments[current_positions[0].y as usize][current_positions[0].x as usize].clone(),
        segments[current_positions[1].y as usize][current_positions[1].x as usize].clone(),
    );

    loop {
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

        if next_positions[0] == next_positions[1] {
            break;
        }
    }

    // Replace the start segment with a standard pipe segment
    // This is necessary to count the segments inside the loop
    segments[start_position.y as usize][start_position.x as usize] = start_segment_as_pipe;

    let segments_inside_loop = count_segments_inside_loop(segments);

    format!(
        "There are {} segments inside the loop",
        segments_inside_loop
    )
}

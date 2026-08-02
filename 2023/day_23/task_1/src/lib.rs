use aoc_utils::{direction::Direction, position::Position};
use std::cmp::max;
use wasm_bindgen::prelude::*;

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, PartialEq, Clone)]
enum Terrain {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone)]
struct Tile {
    terrain: Terrain,
    is_visited: bool,
}

struct Maze {
    tiles: Vec<Tile>,
    bounds: Position,
    end_tile: Position,
}

impl Maze {
    fn new(input: &str) -> Maze {
        let row_len = input.lines().nth(0).unwrap().chars().count();
        let row_count = input.lines().count();
        let bounds = Position {
            row: row_count,
            column: row_len,
        };
        let end_tile = Position {
            row: row_count - 1,
            column: row_len - 2,
        };
        let tiles: Vec<Tile> = input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| Tile {
                    is_visited: false,
                    terrain: match c {
                        '.' => Terrain::Path,
                        '#' => Terrain::Forest,
                        '^' => Terrain::Slope(Direction::Up),
                        'v' => Terrain::Slope(Direction::Down),
                        '<' => Terrain::Slope(Direction::Left),
                        '>' => Terrain::Slope(Direction::Right),
                        _ => panic!("Invalid terrain type: {}", c),
                    },
                })
            })
            .collect();
        Maze {
            tiles,
            bounds,
            end_tile,
        }
    }

    fn get_tile(&self, row: usize, column: usize) -> Option<&Tile> {
        self.tiles.get(row * self.bounds.column + column)
    }

    fn get_mut_tile(&mut self, row: usize, column: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(row * self.bounds.column + column)
    }
}

fn find_longest_path(pos: Position, maze: &mut Maze) -> Option<u64> {
    if pos == maze.end_tile {
        return Some(0);
    }

    let tile = maze.get_mut_tile(pos.row, pos.column).unwrap();
    if tile.is_visited {
        return None;
    }

    tile.is_visited = true;
    let mut longest = None;

    let directions = match &tile.terrain {
        Terrain::Path => ALL_DIRECTIONS.to_vec(),
        Terrain::Forest => vec![],
        Terrain::Slope(slope_dir) => vec![slope_dir.clone()],
    };

    for d in directions {
        let next_pos = d.travel_with_bounds(&pos, &maze.bounds);
        if next_pos.is_some() {
            let next_path = find_longest_path(next_pos.unwrap(), maze);
            longest = match longest {
                Some(current) => match next_path {
                    Some(next_len) => Some(max(current, next_len)),
                    None => longest,
                },
                None => next_path,
            }
        };
    }

    let tile = maze.get_mut_tile(pos.row, pos.column).unwrap();
    tile.is_visited = false;

    match longest {
        Some(len) => Some(len + 1),
        None => None,
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The longest hike is {} steps.", result(input));
}

fn result(input: &str) -> u64 {
    let mut maze = Maze::new(input);
    let start = Position { row: 0, column: 1 };
    return find_longest_path(start, &mut maze).expect("the end cannot be reached");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 94);
    }

    #[test]
    fn it_gets_maze_tiles() {
        let maze = Maze::new("#.\n<>");
        assert_eq!(maze.get_tile(0, 0).unwrap().terrain, Terrain::Forest);
        assert_eq!(maze.get_tile(0, 1).unwrap().terrain, Terrain::Path);
        assert_eq!(
            maze.get_tile(1, 0).unwrap().terrain,
            Terrain::Slope(Direction::Left)
        );
        assert_eq!(
            maze.get_tile(1, 1).unwrap().terrain,
            Terrain::Slope(Direction::Right)
        );
    }
}

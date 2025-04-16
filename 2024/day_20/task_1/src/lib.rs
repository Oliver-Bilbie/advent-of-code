use aoc_utils::{
    direction::*,
    graph::graph::{Edge, Graph},
    position::*,
};
use rayon::prelude::*;

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
    boundary: Position,
    start: Position,
}

#[derive(Clone)]
struct PathItem {
    position: Position,
    distance: u128,
}

impl Maze {
    fn from_str(input: &str) -> Maze {
        let mut start: Option<Position> = None;

        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column, c)| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'S' => {
                            start = Some(Position { row, column });
                            Tile::Empty
                        }
                        'E' => Tile::Empty,
                        _ => panic!("invalid tile character"),
                    })
                    .collect()
            })
            .collect();

        let boundary = Position {
            row: tiles.len(),
            column: tiles.first().unwrap().len(),
        };

        Maze {
            tiles,
            boundary,
            start: start.expect("there to be a start tile"),
        }
    }

    fn get_tile(&self, position: &Position) -> Option<&Tile> {
        Some(self.tiles.get(position.row)?.get(position.column)?)
    }
}

fn build_graph(maze: &Maze) -> Graph<Position> {
    let mut graph = Graph::new();

    for row in 0..maze.boundary.row {
        for column in 0..maze.boundary.column {
            let position = Position { row, column };

            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let source = position.clone();

                if let Some(destination) = direction.travel_with_bounds(&position, &maze.boundary) {
                    if *maze.get_tile(&destination).unwrap() == Tile::Empty {
                        graph.add_edge(Edge {
                            source,
                            destination,
                            weight: 1,
                        });
                    }
                }
            }
        }
    }

    graph
}

fn find_cheat_savings(maze: &Maze) -> Vec<u128> {
    let mut graph = build_graph(&maze);
    graph.dijkstra(maze.start.clone()).unwrap();

    let path: Vec<PathItem> = maze
        .tiles
        .par_iter()
        .enumerate()
        .flat_map(|(row, tiles)| {
            tiles.par_iter().enumerate().filter_map({
                let graph = graph.clone();
                move |(column, tile)| match tile {
                    Tile::Empty => {
                        let position = Position { row, column };
                        let distance = graph.get_node_distance(&position).unwrap();
                        Some(PathItem { position, distance })
                    }
                    Tile::Wall => None,
                }
            })
        })
        .collect();

    let search_path = path.clone();

    path.iter()
        .flat_map(|cheat_start| {
            search_path.iter().filter_map(|cheat_end| {
                if cheat_end.position == cheat_start.position {
                    return None;
                }

                if Position::manhattan_distance(&cheat_end.position, &cheat_start.position) == 2 {
                    if cheat_end.distance > cheat_start.distance + 2 as u128 {
                        return Some(cheat_end.distance - cheat_start.distance - 2 as u128);
                    }
                }

                None
            })
        })
        .collect()
}

pub fn solve(input: &str) -> String {
    let maze = Maze::from_str(&input);
    let cheat_count = find_cheat_savings(&maze)
        .iter()
        .filter(|time_save| **time_save >= 100)
        .count();
    format!(
        "There are {} ways to cheat by at least 100 picoseconds",
        cheat_count
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        time_save: u128,
        count: usize,
    }

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let maze = Maze::from_str(&input);
        let cheat_savings = find_cheat_savings(&maze);

        let test_cases = [
            TestCase {
                time_save: 2,
                count: 44,
            },
            TestCase {
                time_save: 4,
                count: 30,
            },
            TestCase {
                time_save: 6,
                count: 16,
            },
            TestCase {
                time_save: 8,
                count: 14,
            },
            TestCase {
                time_save: 10,
                count: 10,
            },
            TestCase {
                time_save: 12,
                count: 8,
            },
            TestCase {
                time_save: 20,
                count: 5,
            },
            TestCase {
                time_save: 36,
                count: 4,
            },
            TestCase {
                time_save: 38,
                count: 3,
            },
            TestCase {
                time_save: 40,
                count: 2,
            },
            TestCase {
                time_save: 64,
                count: 1,
            },
        ];

        for test_case in test_cases {
            let actual_count = cheat_savings
                .iter()
                .filter(|time_save| **time_save >= test_case.time_save)
                .count();
            assert_eq!(actual_count, test_case.count);
        }
    }
}

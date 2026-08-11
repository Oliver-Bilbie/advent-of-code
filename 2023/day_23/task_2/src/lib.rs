use aoc_utils::{direction::Direction, position::Position};
use std::cmp::max;
use std::collections::HashMap;
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
}

struct Maze {
    tiles: Vec<Terrain>,
    bounds: Position,
    start: Position,
    end: Position,
}

impl Maze {
    fn new(input: &str) -> Maze {
        let row_len = input.lines().nth(0).unwrap().chars().count();
        let row_count = input.lines().count();
        let bounds = Position {
            row: row_count,
            column: row_len,
        };
        let start = Position { row: 0, column: 1 };
        let end = Position {
            row: row_count - 1,
            column: row_len - 2,
        };
        let tiles: Vec<Terrain> = input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '#' => Terrain::Forest,
                    '.' | '^' | 'v' | '<' | '>' => Terrain::Path,
                    _ => panic!("Invalid terrain type: {}", c),
                })
            })
            .collect();
        Maze {
            tiles,
            bounds,
            start,
            end,
        }
    }

    fn get_tile(&self, pos: &Position) -> Option<&Terrain> {
        self.tiles.get(pos.row * self.bounds.column + pos.column)
    }

    fn is_path(&self, pos: &Position) -> bool {
        matches!(self.get_tile(pos), Some(Terrain::Path))
    }

    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|d| {
                let next = d.travel_with_bounds(pos, &self.bounds)?;
                if self.is_path(&next) {
                    Some(next)
                } else {
                    None
                }
            })
            .collect()
    }

    fn junctions(&self) -> Vec<Position> {
        let mut junctions = vec![self.start.clone(), self.end.clone()];
        for row in 0..self.bounds.row {
            for column in 0..self.bounds.column {
                let pos = Position { row, column };
                if !self.is_path(&pos) {
                    continue;
                }
                if pos == self.start || pos == self.end {
                    continue;
                }
                if self.neighbors(&pos).len() > 2 {
                    junctions.push(pos);
                }
            }
        }
        junctions
    }

    fn build_graph(&self) -> (Vec<Position>, Vec<Vec<(usize, u64)>>) {
        let junctions = self.junctions();
        let index: HashMap<Position, usize> = junctions
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect();

        let mut edges: Vec<Vec<(usize, u64)>> = vec![Vec::new(); junctions.len()];

        for (from_idx, from) in junctions.iter().enumerate() {
            for first_step in self.neighbors(from) {
                let mut prev = from.clone();
                let mut curr = first_step;
                let mut dist = 1u64;

                loop {
                    if let Some(&to_idx) = index.get(&curr) {
                        edges[from_idx].push((to_idx, dist));
                        break;
                    }

                    let nexts: Vec<Position> = self
                        .neighbors(&curr)
                        .into_iter()
                        .filter(|n| *n != prev)
                        .collect();

                    if nexts.len() != 1 {
                        break;
                    }

                    prev = curr;
                    curr = nexts.into_iter().next().unwrap();
                    dist += 1;
                }
            }
        }

        (junctions, edges)
    }
}

fn find_longest_path(
    node: usize,
    end: usize,
    edges: &[Vec<(usize, u64)>],
    visited: &mut [bool],
) -> Option<u64> {
    if node == end {
        return Some(0);
    }

    visited[node] = true;
    let mut longest = None;

    for &(next, dist) in &edges[node] {
        if visited[next] {
            continue;
        }
        if let Some(rest) = find_longest_path(next, end, edges, visited) {
            let total = rest + dist;
            longest = Some(match longest {
                Some(current) => max(current, total),
                None => total,
            });
        }
    }

    visited[node] = false;
    longest
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The longest hike is {} steps.", result(input));
}

fn result(input: &str) -> u64 {
    let maze = Maze::new(input);
    let (junctions, edges) = maze.build_graph();
    let start_idx = 0;
    let end_idx = junctions
        .iter()
        .position(|p| *p == maze.end)
        .expect("end junction missing");
    let mut visited = vec![false; junctions.len()];
    find_longest_path(start_idx, end_idx, &edges, &mut visited).expect("the end cannot be reached")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 154);
    }
}

use aoc_utils::{
    direction::ALL_DIRECTIONS,
    graph::graph::{Edge, Graph},
    grid::Grid,
    position::Position,
};
use wasm_bindgen::prelude::*;

fn read_grid(input: &str) -> Grid<usize> {
    let base_grid = Grid::from_str(input, |line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    });
    let mut grid = Grid::<usize>::new(vec![
        vec![0; base_grid.size().column * 5];
        base_grid.size().row * 5
    ]);
    for pos in grid.positions() {
        let base = Position {
            row: pos.row % base_grid.size().row,
            column: pos.column % base_grid.size().column,
        };
        let offset = (pos.row / base_grid.size().row) + (pos.column / base_grid.size().column);
        // we must rescale the risk to 0..=8 so we can use %9 cleanly here
        let risk = (((base_grid.get(&base).unwrap() + offset) - 1) % 9) + 1;
        grid.set(&pos, risk);
    }
    grid
}

fn result(input: &str) -> u64 {
    let grid = read_grid(input);
    let mut graph = Graph::<Position>::new();

    for source in grid.positions() {
        for dir in ALL_DIRECTIONS {
            if let Some(destination) = dir.travel_with_bounds(&source, grid.size()) {
                let weight = *grid.get(&destination).unwrap() as i64;
                graph.add_edge(Edge {
                    source: source.clone(),
                    destination,
                    weight,
                });
            }
        }
    }

    if graph.dijkstra(Position { row: 0, column: 0 }).is_err() {
        panic!("could not compute distances");
    };

    // somehow the yuckiest bit is STILL reading the distance from the graph!
    graph
        .get_node(&Position {
            row: grid.size().row - 1,
            column: grid.size().column - 1,
        })
        .unwrap()
        .min_distance
        .unwrap()
        .try_into()
        .unwrap()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    return format!("The lowest risk is: {}", result(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        assert_eq!(result(&input), 315);
    }
}

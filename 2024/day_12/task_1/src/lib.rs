use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Position {
    row: i16,
    column: i16,
}

impl Position {
    pub fn from_usize(row: usize, column: usize) -> Self {
        Self {
            row: row as i16,
            column: column as i16,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn travel(&self, position: &Position, boundary: &Position) -> Option<Position> {
        match self {
            Direction::North if position.row > 0 => Some(Position {
                row: position.row - 1,
                column: position.column,
            }),
            Direction::East if position.column + 1 < boundary.column => Some(Position {
                row: position.row,
                column: position.column + 1,
            }),
            Direction::South if position.row + 1 < boundary.row => Some(Position {
                row: position.row + 1,
                column: position.column,
            }),
            Direction::West if position.column > 0 => Some(Position {
                row: position.row,
                column: position.column - 1,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Plot {
    plant_type: char,
    size: u32,
    fences: u8,
    visited: bool,
}

#[derive(Clone)]
struct Garden {
    tiles: Vec<Vec<Plot>>,
    boundary: Position,
}

impl Garden {
    fn get_plot(&self, position: &Position) -> Option<&Plot> {
        let row = usize::try_from(position.row).ok()?;
        let column = usize::try_from(position.column).ok()?;
        Some(self.tiles.get(row)?.get(column)?)
    }

    fn get_mut_plot(&mut self, position: &Position) -> Option<&mut Plot> {
        let row = usize::try_from(position.row).ok()?;
        let column = usize::try_from(position.column).ok()?;
        Some(self.tiles.get_mut(row)?.get_mut(column)?)
    }
}

fn read_garden(input: &str) -> Garden {
    let tiles: Vec<Vec<Plot>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Plot {
                    plant_type: c,
                    size: 1,
                    fences: 0,
                    visited: false,
                })
                .collect()
        })
        .collect();

    let boundary = Position::from_usize(tiles.len(), tiles.iter().next().unwrap().len());

    let mut garden = Garden { tiles, boundary };

    evaluate_plot_sizes(&mut garden);
    place_fences(&mut garden);

    garden
}

fn find_neighbors(garden: &Garden, position: &Position) -> Vec<Position> {
    let plant_type = garden.get_plot(&position).unwrap().plant_type;
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .filter_map(|direction| {
        if let Some(neighbor_position) = direction.travel(&position, &garden.boundary) {
            let neighbor = garden.get_plot(&neighbor_position).unwrap();
            if !neighbor.visited && neighbor.plant_type == plant_type {
                return Some(neighbor_position);
            }
        }
        None
    })
    .collect()
}

fn count_neighbors(garden: &mut Garden, position: &Position) -> u32 {
    let neighbors = find_neighbors(garden, position);
    neighbors.iter().for_each(|neighbor_position| {
        let neighbor = garden.get_mut_plot(&neighbor_position).unwrap();
        neighbor.visited = true;
    });

    if neighbors.is_empty() {
        return 0;
    }

    neighbors.len() as u32
        + neighbors
            .iter()
            .map(|neighbor| count_neighbors(garden, neighbor))
            .sum::<u32>()
}

fn evaluate_plot_sizes(garden: &mut Garden) {
    // Clone the original garden so that the original state can be referenced
    let mut garden_input = garden.clone();

    garden
        .tiles
        .iter_mut()
        .enumerate()
        .for_each(|(row, values)| {
            values.iter_mut().enumerate().for_each(|(column, plot)| {
                let cached_size = garden_input
                    .get_plot(&Position::from_usize(row, column))
                    .unwrap()
                    .size;
                if cached_size == 1 {
                    let plot_position = Position::from_usize(row, column);
                    let mut search_garden = garden_input.clone();
                    let neighbor_count = count_neighbors(&mut search_garden, &plot_position);
                    if neighbor_count > 0 {
                        plot.size = neighbor_count;
                        // Set this size for all visited tiles to avoid duplicate calculations
                        search_garden
                            .tiles
                            .iter()
                            .enumerate()
                            .for_each(|(r, values)| {
                                values.iter().enumerate().for_each(|(c, plot)| {
                                    if plot.visited {
                                        garden_input
                                            .get_mut_plot(&Position::from_usize(r, c))
                                            .unwrap()
                                            .size = neighbor_count;
                                    }
                                })
                            });
                    }
                } else {
                    plot.size = cached_size
                }
            })
        });
}

fn place_fences(garden: &mut Garden) {
    // Clone the original garden so that the original state can be referenced
    let garden_input = garden.clone();

    garden
        .tiles
        .iter_mut()
        .enumerate()
        .for_each(|(row, values)| {
            values.iter_mut().enumerate().for_each(|(column, plot)| {
                let plot_position = Position::from_usize(row, column);
                let mut search_garden = garden_input.clone();
                let neighbors = find_neighbors(&mut search_garden, &plot_position);

                plot.fences += 4 - neighbors.len() as u8;
            })
        });
}

fn calculate_fence_cost(garden: &Garden) -> u64 {
    garden
        .tiles
        .iter()
        .map(|row| {
            row.iter()
                .map(|plot| plot.fences as u64 * plot.size as u64)
                .sum::<u64>()
        })
        .sum()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    let garden = read_garden(&input);
    let fence_cost = calculate_fence_cost(&garden);
    format!("The cost of fences is is: {}", fence_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_the_example() {
        let input = std::fs::read_to_string("../test_input.txt").unwrap();
        let actual_solution = solve(&input);
        let expected_solution = "The cost of fences is is: 1930";
        assert_eq!(actual_solution, expected_solution);
    }
}

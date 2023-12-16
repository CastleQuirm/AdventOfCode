// Potential improvements:
//

use std::collections::HashSet;

use crate::directions::CompassDirection::{self, East, North, South, West};
use crate::{coord::Coord2, grid::Grid};

pub fn day16(input_lines: &[Vec<String>]) -> (String, String) {
    let mut maze = Grid::<MirrorType>::from_input(&input_lines[0]);
    maze.add_border(&MirrorType::Edge);

    let answer1 = calculate_active_cells(&maze, Coord2 { x: 1, y: 1 }, East);

    // Brute force Part 2 - there aren't enough possibilities to justify finding anything clever to do.
    // For additional simplicity make use of the fact it's a square grid.
    assert_eq!(input_lines[0].len(), input_lines[0][0].len());
    let core_maze_size =
        TryInto::<i64>::try_into(input_lines[0].len()).expect("Grid size doesn't fit an i64?");
    let answer2 = (1..=core_maze_size)
        .map(|i| {
            calculate_active_cells(&maze, Coord2 { x: 1, y: i }, East)
                .max(calculate_active_cells(
                    &maze,
                    Coord2 {
                        x: core_maze_size,
                        y: i,
                    },
                    West,
                ))
                .max(calculate_active_cells(&maze, Coord2 { x: i, y: 1 }, South))
                .max(calculate_active_cells(
                    &maze,
                    Coord2 {
                        x: i,
                        y: core_maze_size,
                    },
                    North,
                ))
        })
        .max()
        .unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

fn add_next_space(
    activated_spaces: &mut Grid<LightDirections>,
    next_space: Coord2,
    beam_dir: CompassDirection,
    pending_beams: &mut Vec<(Coord2, CompassDirection)>,
) {
    let mut collated_dirs = activated_spaces.get(&next_space).directions;
    // If the new direction is being added to this cell for the first time, update the grid and
    // push a new entry to progress. If it's already there, we don't need to do anything.
    if collated_dirs.insert(beam_dir) {
        activated_spaces.set_cell(
            &next_space,
            &LightDirections {
                directions: collated_dirs,
            },
        );
        pending_beams.push((next_space, beam_dir));
    }
}

fn calculate_active_cells(
    maze: &Grid<MirrorType>,
    starting_coord: Coord2,
    starting_dir: CompassDirection,
) -> usize {
    // Maintain a second grid of the light going through the grid: each space has a Vec of the
    // directions that we know light is travelling out from.
    let mut activated_spaces = Grid {
        grid: vec![vec![LightDirections::none(); maze.grid[0].len()]; maze.grid.len()],
    };
    let east_beam = LightDirections {
        directions: HashSet::from([starting_dir]),
    };
    activated_spaces.set_cell(&starting_coord, &east_beam);

    // Maintain a list of beams we have to progress.
    let mut pending_beams = vec![(starting_coord, starting_dir)];

    // Progress the beams
    while let Some((beam_loc, beam_dir)) = pending_beams.pop() {
        // Create a short local function call to the larger function call using some consistent variables.
        let mut add_next = |beam_dir| {
            add_next_space(
                &mut activated_spaces,
                beam_loc.compass_sum(&beam_dir),
                beam_dir,
                &mut pending_beams,
            );
        };
        // Check where this beam has ended up
        let cell_type = maze.get(&beam_loc);
        // Work out the cell or cells where the beam ends up next based on its direction and
        // what's in its current location
        match (cell_type, beam_dir) {
            (MirrorType::Space, _)
            | (MirrorType::Horizontal, East)
            | (MirrorType::Horizontal, West)
            | (MirrorType::Vertical, North)
            | (MirrorType::Vertical, South) => {
                // continue forwards
                add_next(beam_dir)
            }
            (MirrorType::Edge, _) => (), // Drop
            (MirrorType::Horizontal, _) => {
                assert!(beam_dir == North || beam_dir == South);
                add_next(West);
                add_next(East);
            }
            (MirrorType::Vertical, _) => {
                assert!(beam_dir == East || beam_dir == West);
                add_next(North);
                add_next(South);
            }
            (MirrorType::Forward, North) => add_next(East),
            (MirrorType::Forward, East) => add_next(North),
            (MirrorType::Forward, South) => add_next(West),
            (MirrorType::Forward, West) => add_next(South),
            (MirrorType::Backward, North) => add_next(West),
            (MirrorType::Backward, East) => add_next(South),
            (MirrorType::Backward, South) => add_next(East),
            (MirrorType::Backward, West) => add_next(North),
        }
    }

    // Get the total set of activated spaces. Remember not to include any with type edge.
    activated_spaces
        .filter_elements(&(|dir_set: &LightDirections| !dir_set.directions.is_empty()))
        .iter()
        .filter(|coord| maze.get(coord) != MirrorType::Edge)
        .count()
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum MirrorType {
    Forward,
    Backward,
    Horizontal,
    Vertical,
    Space,
    Edge,
}

impl From<char> for MirrorType {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::Forward,
            '\\' => Self::Backward,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            '.' => Self::Space,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct LightDirections {
    directions: HashSet<CompassDirection>,
}

impl LightDirections {
    fn none() -> Self {
        Self {
            directions: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day16;
    use crate::utils::load_input;

    #[test]
    fn check_day16_case01() {
        full_test(
            ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....", // INPUT STRING
            "46", // PART 1 RESULT
            "51", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day16(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

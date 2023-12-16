// Potential improvements:
//

use std::collections::HashSet;

use crate::directions::CompassDirection::{self, East, North, South, West};
use crate::{coord::Coord2, grid::Grid};

pub fn day16(input_lines: &[Vec<String>]) -> (String, String) {
    let mut maze = Grid::<MirrorType>::from_input(&input_lines[0]);
    maze.add_border(&MirrorType::Edge);

    // Maintain a second grid of the light going through the grid: each space has a Vec of the
    // directions that we know light is travelling out from.
    // Make it two larger in either direction than the original to get the matching co-ords as
    // the version with the border added.
    let mut activated_spaces = Grid {
        grid: vec![
            vec![LightDirections::none(); input_lines[0][0].len() + 2];
            input_lines[0].len() + 2
        ],
    };
    let top_left = Coord2 { x: 1, y: 1 };
    let east_beam = LightDirections {
        directions: HashSet::from([East]),
    };
    activated_spaces.set_cell(&top_left, &east_beam);

    // Maintain a list of beams we have to progress.
    let mut pending_beams = vec![(top_left, East)];
    // let mut steps_run = 0;

    // Progress the beams
    while let Some((beam_loc, beam_dir)) = pending_beams.pop() {
        // steps_run += 1;
        // if steps_run > 100 { break; }
        // println!("{steps_run}: Process beam from {:?}, going direction {:?}", beam_loc, beam_dir);
        let mut add_next = |beam_dir| {
            add_next_space(
                &mut activated_spaces,
                beam_loc.compass_sum(&beam_dir),
                beam_dir,
                &mut pending_beams,
            );
        };
        let cell_type = maze.get(&beam_loc);
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
    let answer1 = activated_spaces
        .filter_elements(&(|dir_set: &LightDirections| !dir_set.directions.is_empty()))
        .iter()
        .filter(|coord| maze.get(coord) != MirrorType::Edge)
        .count();

    let answer2 = 0;
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
            "0",  // PART 2 RESULT
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

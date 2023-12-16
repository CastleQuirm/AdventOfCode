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

    // Progress the beams
    while let Some((beam_loc, beam_dir)) = pending_beams.pop() {
        let cell_type = maze.get(&beam_loc);
        match cell_type {
            MirrorType::Space => {
                // continue forwards
                // TODO commonise some of this for the following branches!
                let next_space = beam_loc.compass_sum(&beam_dir);
                let next_space_current_dirs = activated_spaces.get(&next_space);
                assert!(!next_space_current_dirs.directions.contains(&beam_dir));
                let mut collated_dirs = next_space_current_dirs.directions.clone();
                collated_dirs.insert(beam_dir);
                activated_spaces.set_cell(
                    &next_space,
                    &LightDirections {
                        directions: collated_dirs,
                    },
                );
                pending_beams.push((next_space, beam_dir));
            }
            MirrorType::Edge => (), // Drop
            MirrorType::Horizontal => todo!(),
            MirrorType::Vertical => todo!(),
            MirrorType::Backward => todo!(),
            MirrorType::Forward => todo!(),
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
.|....-|.\
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

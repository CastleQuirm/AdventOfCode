// Potential improvements:
//

use itertools::Itertools;
use std::collections::HashSet;

use grid::{coord::Coord2, Grid};

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    let map = Grid::<char>::from_input(&input_lines[0]);
    let mut antennae_types = HashSet::new();
    map.grid.iter().for_each(|row| {
        row.iter().for_each(|c| {
            antennae_types.insert(c);
        })
    });
    antennae_types.remove(&'.');

    let mut antinodes: HashSet<Coord2> = HashSet::new();
    let mut antinodes_harmonics: HashSet<Coord2> = HashSet::new();

    antennae_types.iter().for_each(|c| {
        for matching_antennae_pair in map.find_elements(c).into_iter().permutations(2) {
            antinodes_harmonics.insert(matching_antennae_pair[1]);
            let delta = matching_antennae_pair[1].diff(&matching_antennae_pair[0]);
            let mut target = matching_antennae_pair[1].sum(&delta);
            if map.in_bounds(&target) {
                antinodes.insert(target);
                antinodes_harmonics.insert(target);
                loop {
                    target = target.sum(&delta);
                    if map.in_bounds(&target) {
                        antinodes_harmonics.insert(target);
                    } else {
                        break;
                    }
                }
            }
        }
    });

    let answer1 = antinodes.len();
    let answer2 = antinodes_harmonics.len();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............", // INPUT STRING
            "14", // PART 1 RESULT
            "34", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day08(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// Potential improvements:
//

use crate::directions::CompassDirection::North;
use crate::grid::Grid;
use itertools::Itertools;

pub fn day14(input_lines: &[Vec<String>]) -> (String, String) {
    let mut grid = Grid::<RockType>::from_input(&input_lines[0]);
    grid.add_border(RockType::Fixed);

    let round_rocks = grid.find_elements(&RockType::Round);
    for rock in round_rocks.iter().sorted_by(|a, b| a.y.cmp(&b.y)) {
        // Find where the rock rolls to
        let mut rolling_location = *rock;
        while grid.get(&rolling_location.compass_sum(&North)) == RockType::Empty {
            rolling_location = rolling_location.compass_sum(&North);
        }
        // Move the rock to its final stopping point
        // TODO make a function for this!
        grid.grid[rock.y as usize][rock.x as usize] = RockType::Empty;
        grid.grid[rolling_location.y as usize][rolling_location.x as usize] = RockType::Round;
    }

    let south_pivot_max_distance = grid.grid.len() - 1;

    let answer1 = grid
        .find_elements(&RockType::Round)
        .iter()
        .map(|rock| south_pivot_max_distance - rock.y as usize)
        .sum::<usize>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RockType {
    Fixed,
    Round,
    Empty,
}

impl From<char> for RockType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Fixed,
            'O' => Self::Round,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day14;
    use crate::utils::load_input;

    #[test]
    fn check_day14_case01() {
        full_test(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....", // INPUT STRING
            "136", // PART 1 RESULT
            "0",   // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day14(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

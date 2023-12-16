// Potential improvements:
//

use std::collections::HashMap;

use crate::coord::Coord2;
use crate::directions::CompassDirection::{self, East, North, South, West};
use crate::grid::Grid;
use itertools::Itertools;

pub fn day14(input_lines: &[Vec<String>]) -> (String, String) {
    let mut grid = Grid::<RockType>::from_input(&input_lines[0]);
    grid.add_border(&RockType::Fixed);

    tilt_board(&mut grid, &North);
    let answer1 = north_load(&grid);

    // Complete the first cycle
    tilt_board(&mut grid, &West);
    tilt_board(&mut grid, &South);
    tilt_board(&mut grid, &East);

    // // Cycle another 999_999_999 times - yeah maybe not
    // for i in 1..1_000_000_000 {
    //     cycle(&mut grid);
    //     if i % 1_000 == 0 {
    //         println!("{:?}", north_load(&grid));
    //     }
    // }
    // let answer2 = north_load(&grid);

    // Cycle until we find a loop
    // **Bold assumption** - we're going to assume that two positions (after a whole set of cycles)
    // with the same north AND west load are the same complete positions
    let mut known_positions = HashMap::new();
    let mut cycles_run = 1;
    while !known_positions.contains_key(&(north_load(&grid), west_load(&grid))) {
        known_positions.insert((north_load(&grid), west_load(&grid)), cycles_run);
        cycle(&mut grid);
        cycles_run += 1;
    }

    // We've found a loop.
    let loop_size = cycles_run
        - known_positions
            .get(&(north_load(&grid), (west_load(&grid))))
            .unwrap();
    let extra_steps = (1_000_000_000 - cycles_run) % loop_size;
    // Could use the lookup table in reverse...or could just run the extra steps.
    for _i in 0..extra_steps {
        cycle(&mut grid);
    }
    let answer2 = north_load(&grid);

    (format!("{}", answer1), format!("{}", answer2))
}

fn tilt_board(grid: &mut Grid<RockType>, direction: &CompassDirection) {
    let round_rocks = grid.find_elements(&RockType::Round);
    let cmp_func = |direction: &CompassDirection, a: &Coord2, b: &Coord2| match direction {
        North => a.y.cmp(&b.y),
        East => b.x.cmp(&a.x),
        South => b.y.cmp(&a.y),
        West => a.x.cmp(&b.x),
    };

    for rock in round_rocks
        .iter()
        .sorted_by(|a, b| cmp_func(direction, a, b))
    {
        // Find where the rock rolls to
        let mut rolling_location = *rock;
        while grid.get(&rolling_location.compass_sum(direction)) == RockType::Empty {
            rolling_location = rolling_location.compass_sum(direction);
        }
        // Move the rock to its final stopping point
        // TODO make a Grid function for substituting the value at a given coord!
        grid.grid[rock.y as usize][rock.x as usize] = RockType::Empty;
        grid.grid[rolling_location.y as usize][rolling_location.x as usize] = RockType::Round;
    }
}

fn cycle(grid: &mut Grid<RockType>) {
    tilt_board(grid, &North);
    tilt_board(grid, &West);
    tilt_board(grid, &South);
    tilt_board(grid, &East);
}

fn north_load(grid: &Grid<RockType>) -> usize {
    let south_pivot_max_distance = grid.grid.len() - 1;
    grid.find_elements(&RockType::Round)
        .iter()
        .map(|rock| south_pivot_max_distance - rock.y as usize)
        .sum::<usize>()
}

fn west_load(grid: &Grid<RockType>) -> usize {
    let south_pivot_max_distance = grid.grid[0].len() - 1;
    grid.find_elements(&RockType::Round)
        .iter()
        .map(|rock| south_pivot_max_distance - rock.x as usize)
        .sum::<usize>()
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
            "64",  // PART 2 RESULT
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

// Potential improvements:
//

use std::collections::HashSet;

use grid::{
    coord::Coord2,
    directions::{CompassDirection, Rotation},
    Grid,
};

pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let garden = Grid::<char>::from_input(&input_lines[0]);
    let mut covered_spaces = HashSet::<Coord2>::new();
    let mut answer1 = 0;
    let mut answer2 = 0;
    (0..garden.height()).for_each(|y| {
        (0..garden.width()).for_each(|x| {
            let start_coord = Coord2::from((x as i64, y as i64));
            if !covered_spaces.contains(&start_coord) {
                let (region, perimeter) = garden.find_region(&start_coord);
                answer1 += perimeter * region.len();
                answer2 += sides(&region) * region.len();
                covered_spaces = covered_spaces
                    .union(&region)
                    .cloned()
                    .collect::<HashSet<_>>();
            }
        });
    });

    (format!("{}", answer1), format!("{}", answer2))
}

fn sides(region: &HashSet<Coord2>) -> usize {
    let mut side_count = 0;
    CompassDirection::iter().for_each(|direction| {
        let mut considered_cells = HashSet::new();
        region.iter().for_each(|cell| {
            // We're now looking at this cell, but don't do anything if we've already looked
            if considered_cells.insert(*cell) {
                // Does this have a side in the considered direction?
                let next_cell = cell.compass_sum(&direction);
                if !region.contains(&next_cell) {
                    side_count += 1;
                    // Now go as far as we can along this edge in each direction
                    explore_in_direction(
                        cell,
                        &next_cell,
                        region,
                        &direction.rotate(&Rotation::Left),
                        &mut considered_cells,
                    );
                    explore_in_direction(
                        cell,
                        &next_cell,
                        region,
                        &direction.rotate(&Rotation::Right),
                        &mut considered_cells,
                    );
                }
            }
        });
    });
    side_count
}

fn explore_in_direction(
    cell: &Coord2,
    next_cell: &Coord2,
    region: &HashSet<Coord2>,
    direction: &CompassDirection,
    considered_cells: &mut HashSet<Coord2>,
) {
    let mut current_in = *cell;
    let mut current_out = *next_cell;
    loop {
        current_in = current_in.compass_sum(direction);
        current_out = current_out.compass_sum(direction);
        if region.contains(&current_in) {
            considered_cells.insert(current_in);
            if region.contains(&current_out) {
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
        full_test(
            "AAAA
BBCD
BBCC
EEEC", // INPUT STRING
            "140", // PART 1 RESULT
            "80",  // PART 2 RESULT
        )
    }

    #[test]
    fn check_day12_case02() {
        full_test(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO", // INPUT STRING
            "772", // PART 1 RESULT
            "436", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day12_case03() {
        full_test(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE", // INPUT STRING
            "1930", // PART 1 RESULT
            "1206", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day12(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

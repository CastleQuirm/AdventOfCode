// Potential improvements:
//

use std::collections::{HashMap, HashSet};

use crate::{coord::Coord2, grid::Grid};

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    // Set up a grid of the input lines. This will be useful for searching for symbols.
    // Add a grid of '.' around it for easier border checks. Note this effectively makes the
    // original grid 1-indexed.
    let mut input_grid = Grid::<char>::from_input(&input_lines[0]);
    input_grid.add_border('.');

    // Create a hash-map of gear co-ordinates and the values of parts next to them.
    let mut gear_map: HashMap<Coord2, Vec<u32>> = HashMap::new();

    // Read the lines looking for numbers.
    let answer1 = input_lines[0]
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let mut sum_of_valid_nums_on_line = 0;
            let mut current_num = 0;
            let mut starting_col = None;
            // Add a dot to the end of each line for processing numbers that finish at the end of the line
            let line = line.to_owned() + ".";
            line.chars().enumerate().for_each(|(col, c)| {
                if let Some(v) = c.to_digit(10) {
                    // Part of a number
                    current_num = current_num * 10 + v;
                    if starting_col.is_none() {
                        starting_col = Some(col);
                    }
                } else {
                    // Not part of a number. Don't care if it's a symbol or not.
                    if let Some(left) = starting_col {
                        // We've just finished a number. See if it's relevant to add, then clean up.
                        let mut surrounding_characters = HashSet::new();
                        check_and_record_gear(
                            &input_grid,
                            Coord2::new(left.try_into().unwrap(), (row + 1).try_into().unwrap()),
                            &mut gear_map,
                            current_num,
                            &mut surrounding_characters,
                        );
                        check_and_record_gear(
                            &input_grid,
                            Coord2::new(
                                (col + 1).try_into().unwrap(),
                                (row + 1).try_into().unwrap(),
                            ),
                            &mut gear_map,
                            current_num,
                            &mut surrounding_characters,
                        );
                        for x in left..(col + 2) {
                            check_and_record_gear(
                                &input_grid,
                                Coord2::new(x.try_into().unwrap(), row.try_into().unwrap()),
                                &mut gear_map,
                                current_num,
                                &mut surrounding_characters,
                            );
                            check_and_record_gear(
                                &input_grid,
                                Coord2::new(x.try_into().unwrap(), (row + 2).try_into().unwrap()),
                                &mut gear_map,
                                current_num,
                                &mut surrounding_characters,
                            );
                        }
                        if surrounding_characters.iter().any(|&c| c != '.') {
                            sum_of_valid_nums_on_line += current_num;
                        }
                        current_num = 0;
                        starting_col = None;
                    }
                }
            });
            sum_of_valid_nums_on_line
        })
        .sum::<u32>();

    let answer2 = gear_map
        .iter()
        .filter_map(|(_, parts)| {
            if parts.len() == 2 {
                Some(parts[0] * parts[1])
            } else {
                None
            }
        })
        .sum::<u32>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn check_and_record_gear(
    input_grid: &Grid<char>,
    check_coord: Coord2,
    gear_map: &mut HashMap<Coord2, Vec<u32>>,
    current_num: u32,
    surrounding_characters: &mut HashSet<char>,
) {
    let adj_char = input_grid.get(check_coord);
    if adj_char == '*' {
        gear_map
            .entry(check_coord)
            .and_modify(|adj_parts| adj_parts.push(current_num))
            .or_insert(vec![current_num]);
    }
    surrounding_characters.insert(adj_char);
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "467..114..
...*.....*
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..", // INPUT STRING
            "4361",   // PART 1 RESULT
            "467835", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

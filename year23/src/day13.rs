// Potential improvements:
//

use std::collections::VecDeque;

pub fn day13(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines.iter().map(|map| find_reflection_value(map)).sum::<usize>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn find_reflection_value(map: &[String]) -> usize {
    // While we've got ready-formed rows, search for a horizontal line of reflection.
    if let Some(row_result) = find_mirror_line(map) {
        return row_result * 100
    }

    // Convert the rows into columns
    let mut columns = Vec::new();
    for i in 0..map[0].len() {
        columns.push(map.iter().map(|row| row.chars().nth(i).unwrap()).collect::<String>());
    }

    // Return the mirror result, which we must find.
    find_mirror_line(&columns).expect("Didn't find a mirror line in rows or columns")
}

fn find_mirror_line(lines: &[String]) -> Option<usize> {
    let mut rows_to_go = lines.iter().cloned().collect::<VecDeque<String>>();
    let mut rows_passed = VecDeque::from([rows_to_go.pop_front().unwrap()]);
    while !rows_to_go.is_empty() {
        let row_count = rows_to_go.len().min(rows_passed.len());
        if (0..row_count).all(|i| rows_to_go[i] == rows_passed[i]) {
            return Some(rows_passed.len())
        }
        rows_passed.push_front(rows_to_go.pop_front().unwrap())
    }
    None
}

#[cfg(test)]
mod tests {
    use super::day13;
    use crate::utils::load_input;

    #[test]
    fn check_day13_case01() {
        full_test(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",  // INPUT STRING
            "405", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day13(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

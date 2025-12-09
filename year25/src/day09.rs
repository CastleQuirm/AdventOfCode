// Potential improvements:
//

use std::str::FromStr;

use grid::coord::Coord2;
use itertools::Itertools;

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    let corners = input_lines[0]
        .iter()
        .map(|line| Coord2::from_str(line).expect("Bad coord"))
        .collect::<Vec<Coord2>>();
    let answer1 = corners
        .iter()
        .combinations(2)
        .map(|pair| {
            let delta = pair[0].diff(pair[1]);
            ((delta.x + 1) * (delta.y + 1)).abs()
        })
        .max()
        .unwrap();

    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case01() {
        full_test(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3", // INPUT STRING
            "50", // PART 1 RESULT
            "0",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day09(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

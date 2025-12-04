// Potential improvements:
//

use grid::coord::{DELTAS_DIAG_ONLY, DELTAS_ORTH_AND_DIAG};
use grid::Grid;

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let mut wordsearch = Grid::<char>::from_input(&input_lines[0]);
    wordsearch.add_border(&'Z');

    let answer1 = wordsearch
        .find_elements(&'X')
        .iter()
        .map(|x| {
            DELTAS_ORTH_AND_DIAG
                .iter()
                .filter(|d| {
                    wordsearch.get(&x.sum(d)) == 'M'
                        && wordsearch.get(&x.sum(d).sum(d)) == 'A'
                        && wordsearch.get(&x.sum(d).sum(d).sum(d)) == 'S'
                })
                .count()
        })
        .sum::<usize>();

    let answer2 = wordsearch
        .find_elements(&'A')
        .iter()
        .filter(|a| {
            DELTAS_DIAG_ONLY
                .iter()
                .filter(|d| {
                    wordsearch.get(&a.sum(d)) == 'M' && wordsearch.get(&a.sum(&d.mult(-1))) == 'S'
                })
                .count()
                == 2
        })
        .count();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX", // INPUT STRING
            "18", // PART 1 RESULT
            "9",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

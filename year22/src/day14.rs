use std::collections::HashMap;

use crate::coord::Coord2;

pub fn day14(input_lines: &str) -> (String, String) {
    let rock = input_lines
        .lines()
        .map(|line| {
            line
                .split(" -> ")
                .map(|c| c.parse::<Coord2>().expect("Didn't parse a coord"))
                .collect::<Vec<Coord2>>()
                .windows(2)
                .map(|w| {
                    let delta = w[0].diff(&w[1]);
                })
                .reduce(|mut line_rock, additional| {
                    line_rock.extend(additional);
                    line_rock
                })
                .unwrap()
        })
        .reduce(|mut all_rock, additional| {
            all_rock.extend(additional);
            all_rock
        })
        .unwrap();
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_day14_part1_case1() {
    //     assert_eq!(day14("").0, "0".to_string())
    // }

    // #[test]
    // fn check_day14_part2_case1() {
    //     assert_eq!(day14("").1, "0".to_string())
    // }

    #[test]
    fn check_day14_both_case1() {
        assert_eq!(
            day14(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            ),
            ("24".to_string(), "0".to_string())
        )
    }
}

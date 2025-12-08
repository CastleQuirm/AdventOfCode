// Potential improvements:
//

use std::str::FromStr;

use grid::coord::Coord3;
use itertools::Itertools;

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    let cables = input_lines[0]
        .iter()
        .map(|line| Coord3::from_str(line).expect("Bad coord"))
        .enumerate()
        .combinations(2)
        .map(|combi| Cabling {
            square_dist: combi[0].1.eucl_dist_squared(&combi[1].1),
            first: combi[0].0,
            second: combi[1].0,
        })
        .sorted()
        .collect::<Vec<Cabling>>();

    // This is an array that gives the lowest indexed junction box that that index of box connects to.
    let mut primaries = input_lines[0]
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let used_conns = if primaries.len() == 20 {
        10 // test code
    } else {
        1000 // real input
    };

    // This updates the set of primaries for each of the used connections.
    cables[0..used_conns].iter().for_each(|cable| {
        let lowest_source = primaries[cable.first].min(primaries[cable.second]);
        let replaced_source = primaries[cable.first].max(primaries[cable.second]);
        primaries = primaries
            .iter()
            .map(|source| {
                if *source == replaced_source {
                    lowest_source
                } else {
                    *source
                }
            })
            .collect::<Vec<usize>>();
    });

    // Now need to count up each possible index for how many use it as a primary, and mutliply the three highest values.
    let mut dependents = vec![0; primaries.len()];
    primaries.iter().for_each(|source| {
        dependents[*source] += 1;
    });
    dependents.sort();
    dependents.reverse();

    let answer1 = dependents[0] * dependents[1] * dependents[2];
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct Cabling {
    square_dist: i64,
    first: usize,
    second: usize,
}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689", // INPUT STRING
            "40", // PART 1 RESULT
            "0",  // PART 2 RESULT
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

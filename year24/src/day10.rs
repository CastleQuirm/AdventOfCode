// Potential improvements:
//

use std::collections::{HashMap, HashSet};

use crate::{
    coord::{Coord2, DELTAS_ORTH_ONLY},
    grid::Grid,
};

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    let topography = Grid::<u32>::from_digits_with_border(&input_lines[0], &99);
    let answer1 = topography
        .find_elements(&0)
        .iter()
        .map(|start| {
            let mut from = HashSet::from([*start]);
            (1..10).for_each(|new_height| {
                from = from
                    .iter()
                    .flat_map(|source| {
                        DELTAS_ORTH_ONLY.iter().filter_map(|movement| {
                            let new_loc = source.sum(movement);
                            if topography.get(&new_loc) == new_height {
                                Some(new_loc)
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<HashSet<Coord2>>();
            });
            from.len()
        })
        .sum::<usize>();
    let answer2 = topography
        .find_elements(&0)
        .iter()
        .map(|start| {
            let mut from = HashMap::from([(*start, 1)]);
            (1..10).for_each(|new_height| {
                let mut new_map = HashMap::new();
                from.iter().for_each(|(source, routes)| {
                    // let routes = *from.get(source).expect("must have been able to come from here?"):
                    DELTAS_ORTH_ONLY.iter().for_each(|movement| {
                        let new_loc = source.sum(movement);
                        if topography.get(&new_loc) == new_height {
                            new_map
                                .entry(new_loc)
                                .and_modify(|e: &mut i32| *e += *routes)
                                .or_insert(*routes);
                        }
                    });
                });
                from = new_map;
            });
            from.values().sum::<i32>()
        })
        .sum::<i32>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day10;
    use crate::utils::load_input;

    #[test]
    fn check_day10_case01() {
        full_test(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732", // INPUT STRING
            "36", // PART 1 RESULT
            "81", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day10(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

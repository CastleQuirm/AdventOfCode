use std::collections::HashSet;

use itertools::{Itertools, MinMaxResult};

use crate::coord::Coord3;

pub fn day18(input_lines: &str) -> (String, String) {
    let lava = input_lines
        .lines()
        .map(|line| line.parse::<Coord3>().unwrap())
        .collect::<HashSet<Coord3>>();
    let answer1 = lava.len() * 6
        - lava
            .iter()
            .combinations(2)
            .filter(|pair| pair[0].manhattan_dist(pair[1]) == 1)
            .count()
            * 2;

    // Find the edges of the lava
    let (min_x, max_x) = match lava.iter().minmax_by(|a, b| a.x.cmp(&b.x)) {
        MinMaxResult::MinMax(min, max) => (min.x, max.x),
        _ => panic!(),
    };
    let (min_y, max_y) = match lava.iter().minmax_by(|a, b| a.y.cmp(&b.y)) {
        MinMaxResult::MinMax(min, max) => (min.y, max.y),
        _ => panic!(),
    };
    let (min_z, max_z) = match lava.iter().minmax_by(|a, b| a.z.cmp(&b.z)) {
        MinMaxResult::MinMax(min, max) => (min.z, max.z),
        _ => panic!(),
    };

    // Start in the lowest corner (moved by another one in each direction) and work out, as though we're 'air'
    let mut exposed_sides = 0;
    let mut known_air = HashSet::from([Coord3::new(min_x - 1, min_y - 1, min_z - 1)]);
    let mut expansion_space = HashSet::from([
        Coord3::new(min_x, min_y - 1, min_z - 1),
        Coord3::new(min_x - 1, min_y, min_z - 1),
        Coord3::new(min_x - 1, min_y - 1, min_z),
    ]);
    let unit_deltas = Coord3::unit_deltas();

    while !expansion_space.is_empty() {
        let mut further_expansion = HashSet::new();
        expansion_space.iter().for_each(|air| {
            // For each of the expansion air, attempt to expand in each of the six directions.
            unit_deltas.iter().for_each(|delta| {
                let new_space = air.sum(delta);
                if new_space.x >= min_x - 1
                    && new_space.x <= max_x + 1
                    && new_space.y >= min_y - 1
                    && new_space.y <= max_y + 1
                    && new_space.z >= min_z - 1
                    && new_space.z <= max_z + 1
                {
                    // In bounds
                    if lava.contains(&new_space) {
                        // We found an edge into the lava!
                        exposed_sides += 1;
                    } else if !(expansion_space.contains(&new_space)
                        || known_air.contains(&new_space)
                        || further_expansion.contains(&new_space))
                    {
                        // We have a new place for air to move into
                        further_expansion.insert(new_space);
                    }
                }
            })
        });
        known_air = known_air
            .union(&expansion_space)
            .cloned()
            .collect::<HashSet<Coord3>>();
        expansion_space = further_expansion;
    }

    (format!("{}", answer1), format!("{}", exposed_sides))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day18_both_case1() {
        assert_eq!(
            day18(
                "1,1,1
2,1,1"
            ),
            ("10".to_string(), "10".to_string())
        );
        assert_eq!(
            day18(
                "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            ),
            ("64".to_string(), "58".to_string())
        )
    }
}

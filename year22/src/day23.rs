use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{coord::Coord2, directions::Direction};

pub fn day23(input_lines: &str) -> (String, String) {
    let mut elves = input_lines
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char == '#' {
                        Some(Coord2::new(x as i64, y as i64))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<Coord2>>()
        })
        .collect::<HashSet<Coord2>>();

    (0..10).for_each(|i| elves = tick(i % 4, &elves));

    let x_diff = match elves.iter().minmax_by_key(|e| e.x) {
        itertools::MinMaxResult::MinMax(min_x, max_x) => max_x.x - min_x.x + 1,
        _ => unreachable!(),
    };

    let y_diff = match elves.iter().minmax_by_key(|e| e.y) {
        itertools::MinMaxResult::MinMax(min_y, max_y) => max_y.y - min_y.y + 1,
        _ => unreachable!(),
    };
    let answer1 = (x_diff * y_diff) - elves.len() as i64;

    let mut answer2 = 10;
    loop {
        answer2 += 1;
        let new_elves = tick((answer2 - 1) % 4, &elves);
        if new_elves == elves {
            break;
        }
        elves = new_elves;
    }

    (format!("{}", answer1), format!("{}", answer2))
}

fn tick(time_mod_4: usize, elves: &HashSet<Coord2>) -> HashSet<Coord2> {
    // Each elf works out a candidate location, don't need to store if self.
    // Store each in a candidate location map pointing to the original location.
    // If the key already exists, remove the entry instead of adding anything.

    // Then form a new set of elves from the original set minus the map's values plus the map's keys.

    let mut candidate_locations = HashMap::new();

    let preferred_directions = Vec::from([
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
        Direction::Left,
    ])[time_mod_4..time_mod_4 + 4]
        .to_vec();

    let checked_deltas = HashMap::from([
        (
            Direction::Up,
            Vec::from([Coord2::new(-1, 1), Coord2::new(0, 1), Coord2::new(1, 1)]),
        ),
        (
            Direction::Down,
            Vec::from([Coord2::new(-1, -1), Coord2::new(0, -1), Coord2::new(1, -1)]),
        ),
        (
            Direction::Right,
            Vec::from([Coord2::new(1, -1), Coord2::new(1, 0), Coord2::new(1, 1)]),
        ),
        (
            Direction::Left,
            Vec::from([Coord2::new(-1, -1), Coord2::new(-1, 0), Coord2::new(-1, 1)]),
        ),
    ]);

    let all_direction_inc_diags = HashSet::from([
        Coord2::new(-1, -1),
        Coord2::new(0, -1),
        Coord2::new(1, -1),
        Coord2::new(-1, 1),
        Coord2::new(0, 1),
        Coord2::new(1, 1),
        Coord2::new(-1, 0),
        Coord2::new(1, 0),
    ]);

    for elf in elves {
        // Work out if it wants to move.
        if all_direction_inc_diags
            .iter()
            .any(|d| elves.contains(&elf.sum(d)))
        {
            // Work out where it wants to move.
            let candidate_location = preferred_directions
                .iter()
                .find(|delta_dir| {
                    !checked_deltas
                        .get(delta_dir)
                        .unwrap()
                        .iter()
                        .any(|d| elves.contains(&elf.sum(d)))
                })
                .map(|direction| elf.sum(&Coord2::movement(direction)));

            if let Some(candidate) = candidate_location {
                // Check if another elf wants to move here.  If not, add this elf.  If so, remove that elf.
                if let std::collections::hash_map::Entry::Vacant(e) =
                    candidate_locations.entry(candidate)
                {
                    e.insert(*elf);
                } else {
                    candidate_locations.remove(&candidate);
                }
            }
        }
    }

    elves
        .difference(
            &candidate_locations
                .values()
                .cloned()
                .collect::<HashSet<_>>(),
        )
        .cloned()
        .collect::<HashSet<_>>()
        .union(&candidate_locations.keys().cloned().collect::<HashSet<_>>())
        .cloned()
        .collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day23_part1_case1() {
        assert_eq!(
            day23(
                ".....
..##.
..#..
.....
..##.
....."
            )
            .0,
            "25".to_string()
        )
    }

    #[test]
    fn check_day23_both_case1() {
        assert_eq!(
            day23(
                "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."
            ),
            ("110".to_string(), "20".to_string())
        )
    }
}

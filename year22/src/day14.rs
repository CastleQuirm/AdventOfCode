use std::collections::HashSet;

use crate::{coord::Coord2, directions::Direction};

pub fn day14(input_lines: &str) -> (String, String) {
    let rock = input_lines
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|c| c.parse::<Coord2>().expect("Didn't parse a coord"))
                .collect::<Vec<Coord2>>()
                .windows(2)
                .map(|w| {
                    let delta_dir = w[1]
                        .cardinal_direction_diff(&w[0])
                        .expect("Link not a cardinal direction");
                    let mut rock_line = HashSet::from([w[0]]);
                    let mut rock_to_add = w[0];
                    while rock_to_add != w[1] {
                        rock_to_add.moved(&Coord2::movement(&delta_dir));
                        rock_line.insert(rock_to_add);
                    }
                    rock_line
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

    // Note that the layout in the puzzle has y increasing as we go DOWN, but my coordinates have y increasing
    // upwards.  How do we fix this?  By actually finding the highest rock and having sand fall upwards.
    let lowest_rock = rock.iter().map(|r| r.y).max().expect("No rock?");

    let mut answer1 = 0;

    let mut sand = HashSet::new();

    loop {
        let mut grain = Coord2::new(500, 0);
        let mut grain_settled = false;
        // Have the grain 'fall'
        while !grain_settled {
            let candidate_fall = grain.sum(&Coord2::movement(&Direction::Up));
            if is_space_empty(&candidate_fall, &rock, &sand) {
                grain.moved(&Coord2::movement(&Direction::Up));
            } else if is_space_empty(
                &candidate_fall.sum(&Coord2::movement(&Direction::Left)),
                &rock,
                &sand,
            ) {
                grain.moved(&Coord2::new(-1, 1));
            } else if is_space_empty(
                &candidate_fall.sum(&Coord2::movement(&Direction::Right)),
                &rock,
                &sand,
            ) {
                grain.moved(&Coord2::new(1, 1));
            } else {
                grain_settled = true;
            }

            if grain.y > lowest_rock {
                if answer1 == 0 {
                    // For part 1, this is falling infinitely and we can stop.
                    answer1 = sand.len();
                }

                // The grain is on the floor so it has setted.
                grain_settled = true;
            }
        }

        sand.insert(grain);
        if grain == Coord2::new(500, 0) {
            break;
        }
    }

    let answer2 = sand.len();
    (format!("{}", answer1), format!("{}", answer2))
}

fn is_space_empty(space: &Coord2, rock: &HashSet<Coord2>, sand: &HashSet<Coord2>) -> bool {
    !rock.contains(space) && !sand.contains(space)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_day14_both_case1() {
        assert_eq!(
            day14(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            ),
            ("24".to_string(), "93".to_string())
        )
    }
}

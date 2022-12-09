use std::{collections::HashSet, str::FromStr};

use crate::{coord::Coord2, directions::Direction};

pub fn day09(input_lines: &str) -> (String, String) {
    let head_moves = input_lines
        .lines()
        .map(|line| line.parse::<HeadMove>().unwrap())
        .collect::<Vec<_>>();

    let head_moves: &[HeadMove] = &head_moves;
    let rope_len = 9;
    let mut tail_visited = HashSet::from([Coord2::new(0, 0)]);
    let mut neck_visited = HashSet::from([Coord2::new(0, 0)]);
    let mut head_location = Coord2::new(0, 0);
    let mut knot_locations = vec![Coord2::new(0, 0); rope_len];

    head_moves.iter().for_each(|instruction| {
        // println!("Apply move: {:?}", instruction);
        let head_delta = Coord2::movement(&instruction.direction);
        (0..instruction.distance).for_each(|_| {
            // println!("- Move!");
            // Move the head
            head_location.moved(&head_delta);
            // Catch up the tail if necessary
            let mut prev_knot_location = head_location;
            knot_locations.iter_mut().for_each(|knot| {
                // println!("Prev knot {:?}; next knot {:?}", prev_knot_location, knot);
                match prev_knot_location.manhattan_dist(knot) {
                    0 | 1 => (), // no-op if the previous knot is still adjacent or over the next knot
                    2 => {
                        // 2 distance could be diagonally adjacent, no-op, or straight line, tail moves straight
                        if prev_knot_location.x == knot.x || prev_knot_location.y == knot.y {
                            // Straight line, we have to move straight.
                            let knot_delta = Coord2::new(
                                (prev_knot_location.x - knot.x) / 2,
                                (prev_knot_location.y - knot.y) / 2,
                            );
                            knot.moved(&knot_delta);
                        }
                    }
                    3 | 4 => {
                        // 3 distance means the tail must need to move diagonally (the head can't get three away in a straight line)
                        assert_ne!(prev_knot_location.x, knot.x);
                        assert_ne!(prev_knot_location.y, knot.y);
                        let knot_delta = Coord2::new(
                            (prev_knot_location.x - knot.x) / (prev_knot_location.x - knot.x).abs(),
                            (prev_knot_location.y - knot.y) / (prev_knot_location.y - knot.y).abs(),
                        );
                        knot.moved(&knot_delta);
                    }
                    _ => panic!(), // Can't get 5 or more away
                }
                // println!("New knot location {:?}", knot);
                prev_knot_location = *knot;
            });

            // Make sure we've counted where the neck and tail are.
            neck_visited.insert(*knot_locations.first().unwrap());
            tail_visited.insert(*knot_locations.last().unwrap());
        })
    });
    (
        format!("{}", neck_visited.len()),
        format!("{}", tail_visited.len()),
    )
}

#[derive(Debug)]
struct HeadMove {
    direction: Direction,
    distance: i64,
}

impl FromStr for HeadMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instruction = s.split_ascii_whitespace();
        Ok(Self {
            direction: instruction.next().unwrap().parse::<Direction>()?,
            distance: instruction.next().unwrap().parse::<i64>().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_larger_example_part2() {
        assert_eq!(
            day09(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )
            .1,
            "36".to_string()
        )
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(
            day09(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            ("13".to_string(), "1".to_string())
        )
    }
}

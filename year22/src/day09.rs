use std::{collections::HashSet, str::FromStr};

use crate::{coord::Coord2, directions::Direction};

pub fn day09(input_lines: &str) -> (String, String) {
    let head_moves = input_lines
        .lines()
        .map(|line| line.parse::<HeadMove>().unwrap())
        .collect::<Vec<_>>();
    let mut tail_visited = HashSet::from([Coord2::new(0, 0)]);

    let mut head_location = Coord2::new(0, 0);
    let mut tail_location = Coord2::new(0, 0);

    head_moves.iter().for_each(|instruction| {
        let head_delta = Coord2::movement(&instruction.direction);
        (0..instruction.distance).for_each(|_| {
            // Move the head
            head_location.moved(&head_delta);
            // Catch up the tail if necessary
            // Check some edge cases
            match head_location.manhattan_dist(&tail_location) {
                0 | 1 => (), // no-op if the head is still adjacent or over the tail
                2 => {
                    // 2 distance could be diagonally adjacent, no-op, or straight line, tail moves straight
                    if head_location.x == tail_location.x || head_location.y == tail_location.y {
                        // Straight line, we have to move straight.  It must be in the same direction as the head moved.
                        tail_location.moved(&head_delta);
                    }
                }
                3 => {
                    // 3 distance means the tail must need to move diagonally (the head can't get three away in a straight line)
                    assert_ne!(head_location.x, tail_location.x);
                    assert_ne!(head_location.y, tail_location.y);
                    let tail_delta = Coord2::new(
                        (head_location.x - tail_location.x)
                            / (head_location.x - tail_location.x).abs(),
                        (head_location.y - tail_location.y)
                            / (head_location.y - tail_location.y).abs(),
                    );
                    tail_location.moved(&tail_delta);
                }
                _ => panic!(), // Can't get 4 or more away
            }
            // Make sure we've counted where the tail is.
            tail_visited.insert(tail_location);
        })
    });

    let answer1 = tail_visited.len();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

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

    // #[test]
    // fn check_day09_part1_case1() {
    //     assert_eq!(day09("").0, "0".to_string())
    // }

    // #[test]
    // fn check_day09_part2_case1() {
    //     assert_eq!(day09("").1, "0".to_string())
    // }

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
            ("13".to_string(), "0".to_string())
        )
    }
}

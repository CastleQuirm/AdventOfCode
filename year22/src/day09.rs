use std::{collections::HashSet, str::FromStr};

use crate::{coord::Coord2, directions::Direction};

pub fn day09(input_lines: &str) -> (String, String) {
    let head_moves = input_lines
        .lines()
        .map(|line| line.parse::<HeadMove>().unwrap())
        .collect::<Vec<_>>();

    (format!("{}", move_rope(&head_moves, 1)), format!("{}", move_rope(&head_moves, 10)))
}

fn move_rope(head_moves: &[HeadMove], rope_len: usize) -> usize {
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
    tail_visited.len()
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

    #[test]
    fn check_day09_larger_example_part2() {
        assert_eq!(day09("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20").1, "36".to_string())
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

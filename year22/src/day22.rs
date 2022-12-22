use std::collections::HashMap;

use itertools::Itertools;

use crate::coord::Coord2;
use crate::directions::{Direction, Rotation};

pub fn day22(input_lines: &str) -> (String, String) {
    // Get the map
    let row_count = input_lines.lines().count() - 2;
    let width = input_lines
        .lines()
        .take(row_count)
        .map(|line| line.len())
        .max()
        .unwrap();
    let mut map = Map {
        map: input_lines
            .lines()
            // .rev()
            // .skip(2)
            .take(row_count)
            .enumerate()
            .flat_map(|(row_number, line)| {
                line.chars().enumerate().map(move |(column, character)| {
                    let coord = Coord2::new((column + 1) as i64, (row_number + 1) as i64);
                    let square = match character {
                        '#' => Square::Wall,
                        '.' => Square::Space,
                        ' ' => Square::Void,
                        _ => unreachable!(),
                    };
                    (coord, square)
                })
            })
            .collect::<HashMap<_, _>>(),
        max_x: width,
        max_y: row_count,
    };

    // Fill out the rest of the HashMap for ease.
    for j in 1..=row_count {
        for i in 1..=width {
            let coord = Coord2::new(i as i64, j as i64);
            map.map.entry(coord).or_insert(Square::Void);
            // let print_char = match map.map.get(&Coord2::new(i as i64, j as i64)).unwrap() {
            //     Square::Space => '.',
            //     Square::Wall => '#',
            //     Square::Void => '~',
            // };
            // print!("{print_char}");
        }
        // println!("");
    }

    // Create the starting position
    let starting_x = input_lines
        .lines()
        .next()
        .unwrap()
        .chars()
        .find_position(|&c| c == '.')
        .expect("Nowhere to start on top row")
        .0
        + 1;
    let mut mover = Mover {
        location: Coord2::new(starting_x as i64, 1),
        facing: Direction::Right,
    };
    // println!("Starting at {:?}", mover);

    // Move around the map
    input_lines
        .lines()
        .last()
        .unwrap()
        .to_string()
        .replace('L', "X ") // Add spaces after each character and swap L for R to account for inverted directions.
        .replace('R', "L ")
        .replace('X', "R")
        .split_ascii_whitespace()
        .for_each(|instruction| {
            // println!("Make a step");
            let rotation = instruction
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<Rotation>()
                .ok();
            let distance_string_len = if rotation.is_some() {
                instruction.len() - 1
            } else {
                instruction.len()
            };

            mover.step(
                &map,
                Step {
                    distance: instruction[0..distance_string_len]
                        .parse::<usize>()
                        .expect("Couldn't parse distance"),
                    rotation,
                },
            );
            // println!("Moved to: {:?}", mover);
        });

    let answer1 = mover.score();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

struct Step {
    distance: usize,
    rotation: Option<Rotation>,
}

#[derive(Debug)]
struct Mover {
    location: Coord2,
    facing: Direction,
}

impl Mover {
    fn step(&mut self, map: &Map, step: Step) {
        // Move forwards
        'all_moves: for _ in 0..step.distance {
            let mut keep_searching = true;
            let mut candidate_location = self.location.sum(&Coord2::movement(&self.facing));
            while keep_searching {
                // println!("- Consider moving to {:?}", candidate_location);
                match map.map.get(&candidate_location) {
                    Some(Square::Space) => {
                        // Found a space. Move into it, and move to next step
                        keep_searching = false;
                        self.location = candidate_location;
                    }
                    Some(Square::Wall) => {
                        // Found a wall.  Don't move and skip the rest of the movement
                        break 'all_moves;
                    }
                    Some(Square::Void) => {
                        // Found a void, keep looking for a candidate
                        candidate_location.moved(&Coord2::movement(&self.facing));
                    }
                    None => {
                        // We've wrapped off the edge.  Need to get the correct wrapped location.
                        match self.facing {
                            Direction::Left => {
                                candidate_location =
                                    Coord2::new(map.max_x as i64, candidate_location.y)
                            }
                            Direction::Right => {
                                candidate_location = Coord2::new(1, candidate_location.y)
                            }
                            Direction::Up => {
                                candidate_location = Coord2::new(candidate_location.x, 1)
                            }
                            Direction::Down => {
                                candidate_location =
                                    Coord2::new(candidate_location.x, map.max_y as i64)
                            }
                        }
                    }
                }
            }
        }

        // Rotate
        if let Some(rotation) = step.rotation {
            self.facing = self.facing.rotate(&rotation);
        }
    }

    fn score(&self) -> i64 {
        let facing_score = match self.facing {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 1, // Up and down are reversed compared to the puzzle setting
            Direction::Down => 3,
        };
        self.location.y * 1000 + self.location.x * 4 + facing_score
    }
}

struct Map {
    map: HashMap<Coord2, Square>,
    max_x: usize,
    max_y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Square {
    Space,
    Wall,
    Void,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_day22_part1_case1() {
    //     assert_eq!(day22("").0, "0".to_string())
    // }

    // #[test]
    // fn check_day22_part2_case1() {
    //     assert_eq!(day22("").1, "0".to_string())
    // }

    #[test]
    fn check_day22_both_case1() {
        assert_eq!(
            day22(
                "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            ),
            ("6032".to_string(), "0".to_string())
        )
    }
}

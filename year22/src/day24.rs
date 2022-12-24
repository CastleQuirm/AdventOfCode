use std::collections::HashSet;

use itertools::Itertools;

use crate::{coord::Coord2, directions::Direction};

pub fn day24(input_lines: &str) -> (String, String) {
    let mut valley = Valley::new(input_lines);
    let (entrance, exit) = (valley.entrance, valley.exit);

    reach_destination(&mut valley, &entrance, &exit);
    let answer1 = valley.time;

    reach_destination(&mut valley, &exit, &entrance);
    reach_destination(&mut valley, &entrance, &exit);
    let answer2 = valley.time;
    (format!("{}", answer1), format!("{}", answer2))
}

fn reach_destination(valley: &mut Valley, start: &Coord2, destination: &Coord2) {
    let candidate_moves = [
        Coord2::movement(&Direction::Up),
        Coord2::movement(&Direction::Down),
        Coord2::movement(&Direction::Left),
        Coord2::movement(&Direction::Right),
        Coord2::new(0, 0),
    ];
    let mut possible_positions = HashSet::from([*start]);
    while !possible_positions.contains(destination) {
        valley.tick();
        possible_positions = possible_positions
            .iter()
            .flat_map(|position| {
                candidate_moves
                    .iter()
                    .filter_map(|delta| {
                        let candidate_space = position.sum(delta);
                        if !valley.blizzard_locations.contains(&candidate_space)
                            && candidate_space.x < valley.width - 1
                            && candidate_space.x > 0
                            && (candidate_space.y < valley.height - 1
                                || candidate_space == valley.entrance)
                            && (candidate_space.y > 0 || candidate_space == valley.exit)
                        {
                            Some(candidate_space)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .collect();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valley {
    time: usize,
    height: i64,
    width: i64,
    entrance: Coord2,
    exit: Coord2,
    blizzards: HashSet<Blizzard>,
    blizzard_locations: HashSet<Coord2>,
}

impl Valley {
    fn new(input_lines: &str) -> Self {
        let height = input_lines.lines().count() as i64;
        let blizzards = input_lines
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        match c {
                            '>' => Some(Direction::Right),
                            '<' => Some(Direction::Left),
                            'v' => Some(Direction::Down),
                            '^' => Some(Direction::Up),
                            '.' | '#' => None,
                            _ => unreachable!(),
                        }
                        .map(|direction| Blizzard {
                            location: Coord2::new(x as i64, y as i64),
                            direction,
                        })
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>();
        Self {
            time: 0,
            height: input_lines.lines().count() as i64,
            width: input_lines.lines().next().unwrap().len() as i64,
            entrance: Coord2::new(
                input_lines
                    .lines()
                    .next()
                    .unwrap()
                    .chars()
                    .find_position(|&c| c == '.')
                    .unwrap()
                    .0 as i64,
                height - 1,
            ),
            exit: Coord2::new(
                input_lines
                    .lines()
                    .last()
                    .unwrap()
                    .chars()
                    .find_position(|&c| c == '.')
                    .unwrap()
                    .0 as i64,
                0,
            ),
            blizzards,
            blizzard_locations: HashSet::new(),
        }
    }

    fn tick(&mut self) {
        self.time += 1;
        self.blizzards = self
            .blizzards
            .iter()
            .map(|blizzard| Blizzard {
                location: match blizzard
                    .location
                    .sum(&Coord2::movement(&blizzard.direction))
                {
                    Coord2 { x, y } if x == 0 => Coord2::new(self.width - 2, y),
                    Coord2 { x, y } if y == 0 => Coord2::new(x, self.height - 2),
                    Coord2 { x, y } if x == self.width - 1 => Coord2::new(1, y),
                    Coord2 { x, y } if y == self.height - 1 => Coord2::new(x, 1),
                    location => location,
                },
                direction: blizzard.direction,
            })
            .collect::<HashSet<_>>();
        self.blizzard_locations = self
            .blizzards
            .iter()
            .map(|blizzard| blizzard.location)
            .collect();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Blizzard {
    location: Coord2,
    direction: Direction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day24_both_case1() {
        assert_eq!(
            day24(
                "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
            ),
            ("18".to_string(), "54".to_string())
        )
    }
}

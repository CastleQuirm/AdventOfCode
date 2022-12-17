use std::collections::HashSet;

use crate::{coord::Coord2, directions::Direction};

pub fn day17(input_lines: &str) -> (String, String) {
    let mut wind_iter = input_lines.chars();

    let mut highest_rock = 0;
    let mut filled_rocks: HashSet<Coord2> = HashSet::new();
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut snapshots = Vec::<Snapshot>::new();
    let mut additional_rocks_to_fall = None;
    let mut rock_count = 0;
    let mut extra_height_from_loop = 0;

    while answer1 * answer2 == 0 {
        let mut falling_rock = Rock {
            shape: Shape::choose_shape(rock_count),
            handle: Coord2::new(2, highest_rock + 4),
        };

        for i in 0.. {
            // Wind blows
            let wind_direction = match wind_iter.next().unwrap_or_else(|| {
                // We've wrapped the input. Take a snapshot of our state and see if it's close enough to another to start iterating.
                let new_snapshot = Snapshot {
                    rocks_dropped: rock_count,
                    max_height: highest_rock,
                    steps_into_new_rock: i,
                    current_shape: falling_rock.shape.clone(),
                    top_ten_rows: filled_rocks
                        .iter()
                        .filter_map(|c| {
                            if c.y >= highest_rock - 10 {
                                Some(Coord2::new(c.x, c.y - highest_rock + 10))
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<Coord2>>(),
                };

                // Assume that there will be a loop after a certain amount of input
                if let Some(matching_snapshot) = snapshots.iter().find(|old| {
                    old.steps_into_new_rock == new_snapshot.steps_into_new_rock
                        && old.current_shape == new_snapshot.current_shape
                        && old.top_ten_rows == new_snapshot.top_ten_rows
                }) {
                    let rocks_in_loop =
                        new_snapshot.rocks_dropped - matching_snapshot.rocks_dropped;

                    let further_full_loops = (1_000_000_000_000 - rock_count) / rocks_in_loop;
                    additional_rocks_to_fall =
                        Some((1_000_000_000_000 - rock_count) % rocks_in_loop);

                    extra_height_from_loop = (new_snapshot.max_height
                        - matching_snapshot.max_height)
                        * further_full_loops as i64;
                } else {
                    snapshots.push(new_snapshot);
                }

                wind_iter = input_lines.chars();
                wind_iter.next().unwrap()
            }) {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Unrecognised wind character"),
            };

            if (wind_direction == Direction::Left && falling_rock.handle.x > 0)
                || (wind_direction == Direction::Right && falling_rock.rightmost_spot() < 6)
            {
                // Space to blow between the walls.
                if falling_rock
                    .all_spaces()
                    .iter()
                    .map(|c| c.sum(&Coord2::movement(&wind_direction)))
                    .all(|c| !filled_rocks.contains(&c))
                {
                    // Space to blow between the other rocks - the rock moves!
                    falling_rock
                        .handle
                        .moved(&Coord2::movement(&wind_direction));
                }
            }

            // Rock falls
            if falling_rock.handle.y > 1
                && falling_rock
                    .all_spaces()
                    .iter()
                    .map(|c| c.sum(&Coord2::movement(&Direction::Down)))
                    .all(|c| !filled_rocks.contains(&c))
            {
                // Space to fall - the rock moves!
                falling_rock
                    .handle
                    .moved(&Coord2::movement(&Direction::Down));
            } else {
                // The rock has landed!
                falling_rock.all_spaces().iter().for_each(|c| {
                    filled_rocks.insert(*c);
                });
                highest_rock = i64::max(highest_rock, falling_rock.topmost_spot());
                if rock_count == 2021 {
                    answer1 = highest_rock;
                }

                break;
            }
        }
        rock_count += 1;
        additional_rocks_to_fall = if let Some(additional_rocks) = additional_rocks_to_fall {
            if additional_rocks == 1 {
                answer2 = highest_rock + extra_height_from_loop;
                None
            } else {
                Some(additional_rocks - 1)
            }
        } else {
            None
        };
    }

    (format!("{}", answer1), format!("{}", answer2))
}

struct Snapshot {
    rocks_dropped: usize,
    max_height: i64,
    steps_into_new_rock: usize,
    current_shape: Shape,
    top_ten_rows: HashSet<Coord2>,
}

struct Rock {
    shape: Shape,
    handle: Coord2, // note the handle isn't part of the shape for the plus!
}

impl Rock {
    fn all_spaces(&self) -> Vec<Coord2> {
        self.shape
            .all_relative_coords()
            .iter()
            .map(|delta| self.handle.sum(delta))
            .collect::<Vec<_>>()
    }

    fn rightmost_spot(&self) -> i64 {
        self.handle.x + self.shape.highest_delta_x_from_handle()
    }

    fn topmost_spot(&self) -> i64 {
        self.handle.y + self.shape.highest_delta_y_from_handle()
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Shape {
    Horizontal,
    Plus,
    BackwardsL,
    Vertical,
    Square,
}

impl Shape {
    fn all_relative_coords(&self) -> Vec<Coord2> {
        match self {
            Shape::Horizontal => Vec::from([
                Coord2::new(0, 0),
                Coord2::new(1, 0),
                Coord2::new(2, 0),
                Coord2::new(3, 0),
            ]),
            Shape::Plus => Vec::from([
                Coord2::new(1, 0),
                Coord2::new(0, 1),
                Coord2::new(1, 1),
                Coord2::new(2, 1),
                Coord2::new(1, 2),
            ]),
            Shape::BackwardsL => Vec::from([
                Coord2::new(0, 0),
                Coord2::new(1, 0),
                Coord2::new(2, 0),
                Coord2::new(2, 1),
                Coord2::new(2, 2),
            ]),
            Shape::Vertical => Vec::from([
                Coord2::new(0, 0),
                Coord2::new(0, 1),
                Coord2::new(0, 2),
                Coord2::new(0, 3),
            ]),
            Shape::Square => Vec::from([
                Coord2::new(0, 0),
                Coord2::new(1, 0),
                Coord2::new(0, 1),
                Coord2::new(1, 1),
            ]),
        }
    }

    fn highest_delta_x_from_handle(&self) -> i64 {
        match self {
            Shape::Horizontal => 3,
            Shape::Plus | Shape::BackwardsL => 2,
            Shape::Vertical => 0,
            Shape::Square => 1,
        }
    }

    fn highest_delta_y_from_handle(&self) -> i64 {
        match self {
            Shape::Horizontal => 0,
            Shape::Plus | Shape::BackwardsL => 2,
            Shape::Vertical => 3,
            Shape::Square => 1,
        }
    }

    fn choose_shape(count: usize) -> Self {
        match count % 5 {
            0 => Self::Horizontal,
            1 => Self::Plus,
            2 => Self::BackwardsL,
            3 => Self::Vertical,
            4 => Self::Square,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day17_both_case1() {
        assert_eq!(
            day17(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"),
            ("3068".to_string(), "1514285714288".to_string())
        )
    }
}

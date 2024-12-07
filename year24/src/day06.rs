// Potential improvements:
//

use std::collections::HashSet;

use crate::{coord::Coord2, directions::CompassDirection, grid::Grid};

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let mut map =
        Grid::<MapLocation>::from_input_with_border(&input_lines[0], &MapLocation::OutOfBounds);
    let mut guard_direction = CompassDirection::North;
    let mut guard_location = map
        .find_single_element(&MapLocation::Visited {
            direction: HashSet::from([CompassDirection::North]),
        })
        .expect("Couldn't find starting point");

    let mut potential_added_obstacles = HashSet::new();

    while map.get(&guard_location) != MapLocation::OutOfBounds {
        let next_location = guard_location.compass_sum(&guard_direction);
        match map.get(&next_location) {
            MapLocation::Obstacle => {
                guard_direction = guard_direction.rotate(&crate::directions::Rotation::Right)
            }
            MapLocation::Unvisited if !potential_added_obstacles.contains(&next_location) => {
                // Spin off alternative reality where we've placed an obstacle
                let mut new_map = map.clone();
                new_map.set_cell(&next_location, &MapLocation::Obstacle);
                if test_obstacle(&mut new_map, &guard_location, &guard_direction) {
                    potential_added_obstacles.insert(next_location);
                }
                let (new_cell_val, _) = map
                    .get(&guard_location)
                    .visit_cell_is_new_direction(&guard_direction);
                map.set_cell(&guard_location, &new_cell_val);
                guard_location = next_location;
            }
            _ => {
                let (new_cell_val, _) = map
                    .get(&guard_location)
                    .visit_cell_is_new_direction(&guard_direction);
                map.set_cell(&guard_location, &new_cell_val);
                guard_location = next_location;
            }
        }
    }

    let answer1 = map
        .filter_elements(&(|t: &MapLocation| matches!(t, MapLocation::Visited { direction: _ })))
        .len();
    let answer2 = potential_added_obstacles.len();
    (format!("{}", answer1), format!("{}", answer2))
}

fn test_obstacle(
    map: &mut Grid<MapLocation>,
    init_guard_location: &Coord2,
    init_guard_direction: &CompassDirection,
) -> bool {
    let mut guard_location = *init_guard_location;
    let mut guard_direction = *init_guard_direction;
    while map.get(&guard_location) != MapLocation::OutOfBounds {
        let next_location = guard_location.compass_sum(&guard_direction);
        match map.get(&next_location) {
            MapLocation::Obstacle => {
                guard_direction = guard_direction.rotate(&crate::directions::Rotation::Right)
            }
            _ => {
                let (new_cell_val, in_loop) = map
                    .get(&guard_location)
                    .visit_cell_is_new_direction(&guard_direction);
                if in_loop {
                    return true;
                }
                map.set_cell(&guard_location, &new_cell_val);
                guard_location = next_location;
            }
        }
    }

    false
}

#[derive(Clone, Eq, PartialEq)]
enum MapLocation {
    Unvisited,
    Visited {
        direction: HashSet<CompassDirection>,
    },
    Obstacle,
    OutOfBounds,
}

impl MapLocation {
    fn visit_cell_is_new_direction(&self, new_direction: &CompassDirection) -> (Self, bool) {
        if let MapLocation::Visited { direction } = self {
            if direction.contains(new_direction) {
                (self.clone(), true)
            } else {
                let mut visited_directions = direction.clone();
                visited_directions.insert(*new_direction);
                (
                    MapLocation::Visited {
                        direction: visited_directions,
                    },
                    false,
                )
            }
        } else {
            (
                MapLocation::Visited {
                    direction: HashSet::from([*new_direction]),
                },
                false,
            )
        }
    }
}

impl From<char> for MapLocation {
    fn from(value: char) -> Self {
        match value {
            '#' => MapLocation::Obstacle,
            '.' => MapLocation::Unvisited,
            '^' => MapLocation::Visited {
                direction: HashSet::from([CompassDirection::North]),
            },
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...", // INPUT STRING
            "41", // PART 1 RESULT
            "6",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day06(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

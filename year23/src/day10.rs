// Potential improvements:
//

use std::collections::HashSet;

use crate::coord::Coord2;
use crate::directions::CompassDirection::{self, East, North, South, West};
use crate::grid::Grid;

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    // Build the map and add a border of plain ground.
    let mut map = Grid::<PipeType>::from_input(&input_lines[0]);
    map.add_border(&PipeType::Ground);

    // Find the starting point.
    let starting_coord = map
        .find_single_element(&PipeType::Start)
        .expect("Didn't find a starting point");

    // Find how many possible directions we can go in from the start.
    let possible_directions = possible_next_directions(&map, &starting_coord);

    // If there's more than 2 directions we'd have to do something complex. For now, assume not.
    // (My input doesn't have more than 2 and I assume none will so this is 'legit').
    assert_eq!(possible_directions.len(), 2);
    let mut possible_directions = possible_directions.iter();
    let first_possible_direction = possible_directions.next().unwrap();
    let second_possible_direction = possible_directions.next().unwrap();
    assert!(possible_directions.next().is_none());
    let start_pipe_type = match (*first_possible_direction, *second_possible_direction) {
        (North, East) | (East, North) => PipeType::NE,
        (North, South) | (South, North) => PipeType::NS,
        (North, West) | (West, North) => PipeType::NW,
        (East, South) | (South, East) => PipeType::SE,
        (East, West) | (West, East) => PipeType::EW,
        (South, West) | (West, South) => PipeType::SW,
        _ => panic!(),
    };

    // Walk our way around the loop, counting steps (for Part 1) and building a HashSet of the loop
    // (for part 2).
    let mut num_steps = 1;
    let mut loop_cells = HashSet::from([starting_coord]);
    let mut current_location = starting_coord.compass_sum(first_possible_direction);
    let mut current_direction = *first_possible_direction;
    while map.get(&current_location) != PipeType::Start {
        num_steps += 1;
        loop_cells.insert(current_location);
        (current_location, current_direction) =
            next_coord(&map, &current_location, &current_direction);
    }
    assert_eq!(num_steps % 2, 0);

    // walk every cell of the grid. Within each row, use a boolean to track if we're inside
    // or outside of the pipe, which flip-flops every time we enter a space of the loop,
    // and count the total spaces inside the loop.
    let mut cells_inside_loop = 0;
    for y in 0..map.grid.len() {
        let mut inside_pipe = false;
        let mut incoming_direction_to_horizontal: Option<CompassDirection> = None;
        for x in 0..map.grid[y].len() {
            let cell = Coord2 {
                x: x as i64,
                y: y as i64,
            };
            if loop_cells.contains(&cell) {
                let this_cell = match map.get(&cell) {
                    PipeType::Start => start_pipe_type,
                    PipeType::Ground => panic!(),
                    other => other,
                };
                match (this_cell, incoming_direction_to_horizontal) {
                    (PipeType::NS, None) => inside_pipe = !inside_pipe,
                    (PipeType::NS, Some(_)) => panic!("Can't be along pipe and reach a vert"),
                    (PipeType::NE, None) => incoming_direction_to_horizontal = Some(North),
                    (PipeType::NE, Some(_)) => panic!("Can't be along pipe and not have a W"),
                    (PipeType::NW, None) => panic!("Can't be not in pipe and have a W incoming"),
                    (PipeType::NW, Some(x)) => {
                        incoming_direction_to_horizontal = None;
                        match x {
                            North => (),
                            South => inside_pipe = !inside_pipe,
                            East | West => panic!(),
                        }
                    }
                    (PipeType::SE, None) => incoming_direction_to_horizontal = Some(South),
                    (PipeType::SE, Some(_)) => panic!("Can't be along pipe and not have a W"),
                    (PipeType::SW, None) => panic!("Can't be not in pipe and have a W incoming"),
                    (PipeType::SW, Some(x)) => {
                        incoming_direction_to_horizontal = None;
                        match x {
                            North => inside_pipe = !inside_pipe,
                            South => (),
                            East | West => panic!(),
                        }
                    }
                    (PipeType::EW, None) => panic!("Can't be not in pipe and have a W incoming"),
                    (PipeType::EW, Some(_)) => (),
                    _ => panic!("Bad pipe type"),
                }
            } else if inside_pipe {
                cells_inside_loop += 1;
            }
        }
    }

    let answer1 = num_steps / 2;
    let answer2 = cells_inside_loop;
    (format!("{}", answer1), format!("{}", answer2))
}

fn possible_next_directions(map: &Grid<PipeType>, from_loc: &Coord2) -> HashSet<CompassDirection> {
    // Ideally I'd like to create an iteratable list of directions and their matching checks but function
    // pointers are tricky to get the syntax right.
    // let mut directions_and_checks: Vec<(CompassDirection, &dyn Fn(PipeType) -> bool)> = Vec::new();
    // directions_and_checks.push((North, &PipeType::connects_south));

    let mut possible_directions: HashSet<CompassDirection> = HashSet::new();
    if map.get(&from_loc.compass_sum(&North)).connects_south() {
        possible_directions.insert(North);
    }
    if map.get(&from_loc.compass_sum(&South)).connects_north() {
        possible_directions.insert(South);
    }
    if map.get(&from_loc.compass_sum(&East)).connects_west() {
        possible_directions.insert(East);
    }
    if map.get(&from_loc.compass_sum(&West)).connects_east() {
        possible_directions.insert(West);
    }

    possible_directions
}

// fn continue_next_direction(map: Grid<PipeType>, from_loc: &Coord2, prev_direction: &CompassDirection) -> CompassDirection {
//     // Could do the following to find a next direction, but we should have it from the actual map detail
//     let possible_directions = possible_next_directions(map, from_loc);
//     assert_eq!(possible_directions.len(), 2);
//     if possible_directions[0] == prev_direction.opposite() {
//         possible_directions[1]
//     } else {
//         assert_eq!(possible_directions[1], prev_direction.opposite());
//         possible_directions[0]
//     }
// }

/// Get the coordinate and direction we moved in to get to it by following the pipe.
fn next_coord(
    map: &Grid<PipeType>,
    from_loc: &Coord2,
    prev_direction: &CompassDirection,
) -> (Coord2, CompassDirection) {
    let next_direction = match (map.get(from_loc), prev_direction) {
        (PipeType::NS, North) => North,
        (PipeType::NS, South) => South,
        (PipeType::NE, South) => East,
        (PipeType::NE, West) => North,
        (PipeType::NW, East) => North,
        (PipeType::NW, South) => West,
        (PipeType::SE, North) => East,
        (PipeType::SE, West) => South,
        (PipeType::SW, North) => West,
        (PipeType::SW, East) => South,
        (PipeType::EW, East) => East,
        (PipeType::EW, West) => West,
        (PipeType::Start, _) => panic!("Don't use this function from the starting point!"),
        (PipeType::Ground, _) => panic!("We've ended up outside the pipe!"),
        _ => panic!("Can't get into this pipe from this direction"),
    };
    (from_loc.compass_sum(&next_direction), next_direction)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PipeType {
    NS,
    NE,
    NW,
    SE,
    SW,
    EW,
    Ground,
    Start,
}

impl PipeType {
    fn connects_north(&self) -> bool {
        *self == Self::NS || *self == Self::NE || *self == Self::NW
    }

    fn connects_east(&self) -> bool {
        *self == Self::EW || *self == Self::NE || *self == Self::SE
    }

    fn connects_south(&self) -> bool {
        *self == Self::NS || *self == Self::SE || *self == Self::SW
    }

    fn connects_west(&self) -> bool {
        *self == Self::NW || *self == Self::EW || *self == Self::SW
    }
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day10;
    use crate::utils::load_input;

    #[test]
    fn check_day10_case01() {
        full_test(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF", // INPUT STRING
            "4", // PART 1 RESULT
            "1", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day10_case02() {
        full_test(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ", // INPUT STRING
            "8", // PART 1 RESULT
            "1", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day10_case03() {
        full_test(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........", // INPUT STRING
            "23", // PART 1 RESULT
            "4",  // PART 2 RESULT
        )
    }

    #[test]
    fn check_day10_case04() {
        full_test(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L", // INPUT STRING
            "80", // PART 1 RESULT
            "10", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day10(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

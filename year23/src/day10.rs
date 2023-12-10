// Potential improvements:
//

use crate::coord::Coord2;
use crate::directions::CompassDirection::{self, East, North, South, West};
use crate::grid::Grid;

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    // Build the map and add a border of plain ground.
    let mut map = Grid::<PipeType>::from_input(&input_lines[0]);
    map.add_border(PipeType::Ground);

    // Find the starting point.
    let starting_coord = map
        .find_single_element(&PipeType::Start)
        .expect("Didn't find a starting point");

    // Find how many possible directions we can go in from the start.
    let possible_directions = possible_next_directions(&map, &starting_coord);

    // If there's more than 2 directions we'd have to do something complex. For now, assume not.
    // (My input doesn't have more than 2 and I assume none will so this is 'legit').
    assert_eq!(possible_directions.len(), 2);

    let mut num_steps = 1;
    let mut current_location = starting_coord.compass_sum(&possible_directions[0]);
    let mut current_direction = possible_directions[0];
    while map.get(&current_location) != PipeType::Start {
        num_steps += 1;
        (current_location, current_direction) =
            next_coord(&map, &current_location, &current_direction);
    }
    assert_eq!(num_steps % 2, 0);

    let answer1 = num_steps / 2;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn possible_next_directions(map: &Grid<PipeType>, from_loc: &Coord2) -> Vec<CompassDirection> {
    // Ideally I'd like to create an iteratable list of directions and their matching checks but function
    // pointers are tricky to get the syntax right.
    // let mut directions_and_checks: Vec<(CompassDirection, &dyn Fn(PipeType) -> bool)> = Vec::new();
    // directions_and_checks.push((North, &PipeType::connects_south));

    let mut possible_directions: Vec<CompassDirection> = Vec::new();
    if map.get(&from_loc.compass_sum(&North)).connects_south() {
        // println!("element up: {:?} at coord: {:?}", map.get(starting_coord.sum(&Coord2::movement(&CompassDirection::North))), starting_coord.sum(&Coord2::movement(&CompassDirection::North)));
        possible_directions.push(North);
    }
    if map.get(&from_loc.compass_sum(&South)).connects_north() {
        possible_directions.push(South);
    }
    if map.get(&from_loc.compass_sum(&East)).connects_west() {
        possible_directions.push(East);
    }
    if map.get(&from_loc.compass_sum(&West)).connects_east() {
        possible_directions.push(West);
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
            "0", // PART 2 RESULT
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
            "0", // PART 2 RESULT
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

use std::ops::RangeInclusive;

use crate::coords::Coord2;
use itertools::Itertools;

// Potential improvements:
//

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    // Map a line into a vec of the corner coords.
    let wires = input_lines[0]
        .iter()
        .map(|line| create_wire_corners(line))
        .collect::<Vec<Vec<Coord2>>>();
    assert_eq!(wires.len(), 2);

    // Find the set of intersections
    let mut segments = wires.iter().map(|wire| {
        wire.iter()
            .tuple_windows::<(_, _)>()
            .map(segment_details)
            .collect::<Vec<_>>()
    });
    let segments1 = segments.next().unwrap();
    let segments2 = segments.next().unwrap();
    assert!(segments.next().is_none());

    let intersections = segments1
        .iter()
        .flat_map(|segment1| {
            segments2
                .iter()
                .filter_map(|segment2| wire_intersection(segment1, segment2))
                .collect::<Vec<Coord2>>()
        })
        .collect::<Vec<Coord2>>();

    // Find the one closest to the origin.
    let origin = Coord2::new(0, 0);
    let answer1 = intersections
        .iter()
        .map(|intersection| intersection.manhattan_dist(&origin))
        .min()
        .expect("No intersections");

    // Find the intersection with the shortest travel value to reach it.

    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn create_wire_corners(instructions: &str) -> Vec<Coord2> {
    let mut wire_corners = vec![Coord2::new(0, 0)];
    for step in instructions.split(',') {
        let last_corner = wire_corners.last().unwrap();
        let distance = &step[1..].parse::<i64>().expect("Couldn't parse number");
        let next_corner = match step.char_indices().next().expect("instruction was empty").1 {
            'L' => last_corner.sum(&Coord2::new(-distance, 0)),
            'R' => last_corner.sum(&Coord2::new(*distance, 0)),
            'U' => last_corner.sum(&Coord2::new(0, *distance)),
            'D' => last_corner.sum(&Coord2::new(0, -distance)),
            _ => unreachable!(),
        };
        wire_corners.push(next_corner)
    }

    wire_corners
}

fn wire_intersection(segment1: &SegmentDetails, segment2: &SegmentDetails) -> Option<Coord2> {
    if segment1.direction != segment2.direction
        && segment1.ranged_coord.contains(&segment2.fixed_coord)
        && segment2.ranged_coord.contains(&segment1.fixed_coord)
    {
        let intersection = if segment1.direction == Direction::Horizontal {
            Coord2::new(segment2.fixed_coord, segment1.fixed_coord)
        } else {
            Coord2::new(segment1.fixed_coord, segment2.fixed_coord)
        };
        if intersection != Coord2::new(0, 0) {
            Some(intersection)
        } else {
            None
        }
    } else {
        None
    }
}

fn segment_details(segment: (&Coord2, &Coord2)) -> SegmentDetails {
    if segment.0.get_x() == segment.1.get_x() {
        let min_range = segment.0.get_y().min(segment.1.get_y());
        let max_range = segment.0.get_y().max(segment.1.get_y());

        SegmentDetails {
            direction: Direction::Vertical,
            fixed_coord: segment.0.get_x(),
            ranged_coord: min_range..=max_range,
        }
    } else {
        assert_eq!(segment.0.get_y(), segment.1.get_y());
        let min_range = segment.0.get_x().min(segment.1.get_x());
        let max_range = segment.0.get_x().max(segment.1.get_x());

        SegmentDetails {
            direction: Direction::Horizontal,
            fixed_coord: segment.0.get_y(),
            ranged_coord: min_range..=max_range,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct SegmentDetails {
    direction: Direction,
    fixed_coord: i64,
    ranged_coord: RangeInclusive<i64>,
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "R8,U5,L5,D3
U7,R6,D4,L4", // INPUT STRING
            "6",  // PART 1 RESULT
            "30", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day03_case02() {
        full_test(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83", // INPUT STRING
            "159", // PART 1 RESULT
            "610", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day03_case03() {
        full_test(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", // INPUT STRING
            "135", // PART 1 RESULT
            "410", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

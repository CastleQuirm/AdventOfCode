use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;

use crate::coord::Coord2;

pub fn day15(input_lines: &str) -> (String, String) {
    // Hacky way to judge if this is test or real:
    let y_val = if input_lines.lines().count() < 15 {
        10
    } else {
        2_000_000
    };

    let sensor_beacon_list = input_lines
        .lines()
        .map(parse_sensor_beacon)
        .collect::<Vec<_>>();
    let mut x_ranges = sensor_beacon_list
        .iter()
        .filter_map(|objects| determine_beacon_free_range(objects, y_val))
        .collect::<Vec<_>>();
    x_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let beacons = sensor_beacon_list
        .iter()
        .map(|(_, beacon)| *beacon)
        .collect::<HashSet<_>>();
    let beacons_on_y_line = beacons.iter().filter(|beacon| beacon.y == y_val).count() as i64;

    let x_ranges = x_ranges.iter().fold(
        Vec::<RangeInclusive<i64>>::new(),
        |mut cumulative_ranges, new_range| {
            if let Some(prev) = cumulative_ranges.last_mut() {
                // There's a previous element to fold in with
                if *prev.end() >= new_range.start() - 1 {
                    // The previous element is overlapping
                    *prev = *prev.start()..=i64::max(*new_range.end(), *prev.end());
                } else {
                    cumulative_ranges.push(new_range.clone());
                }
            } else {
                cumulative_ranges.push(new_range.clone());
            }

            cumulative_ranges
        },
    );

    let answer1 = x_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i64>()
        - beacons_on_y_line;

    let distress_beacon = if y_val == 2_000_000 {
        find_distress_beacon(sensor_beacon_list)
    } else {
        Coord2::new(0, 0)
    };

    let answer2 = distress_beacon.x * 4_000_000 + distress_beacon.y;
    (format!("{}", answer1), format!("{}", answer2))
}

fn find_distress_beacon(sensor_beacon_list: Vec<(Coord2, Coord2)>) -> Coord2 {
    // For Part 2: need to find the only uncovered spot with x and y in 0..4_000_000.
    // Find pairs of sensors that have a manhattan distance between them that's the sum of their respective beacon ranges plus two (for the gap in the paired lines).
    // ALSO... only do this for the real code, because the test gets unhappy.
    // This is because we're making a wild assertion that there's only two pairs!  (In the test code, that's not true.  In the real case, it is.)
    // Pick the left element of each pair.
    let mut left_sb_elements = sensor_beacon_list
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            assert_eq!(pair.len(), 2);
            let (sensor_1, beacon_1) = pair[0];
            let (sensor_2, beacon_2) = pair[1];
            if sensor_1.manhattan_dist(sensor_2)
                == sensor_1.manhattan_dist(beacon_1) + sensor_2.manhattan_dist(beacon_2) + 2
            {
                if pair[0].0.x < pair[1].0.x {
                    Some(pair[0])
                } else {
                    Some(pair[1])
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Find the upper and lower element of the the two.
    left_sb_elements.sort_by(|a, b| a.0.y.cmp(&b.0.y));

    // The distress signal will be up-right of the first of these elements!  Search that line for where it intersects the lower-right line from the second one.
    let (ll_sensor, ll_beacon) = left_sb_elements[0];
    let (ul_sensor, ul_beacon) = left_sb_elements[1];
    let ll_radius = ll_sensor.manhattan_dist(ll_beacon) + 1;
    let ul_radius = ul_sensor.manhattan_dist(ul_beacon) + 1;

    (0..=ll_radius)
        .map(|diagonal_dist| ll_sensor.plus(diagonal_dist, ll_radius - diagonal_dist))
        .find(|candidate_distress| ul_sensor.manhattan_dist(candidate_distress) == ul_radius)
        .expect("No distress found")
}

static RE: OnceCell<Regex> = OnceCell::new();

fn parse_sensor_beacon(line: &str) -> (Coord2, Coord2) {
    RE.get_or_init(|| {
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap()
    })
    .captures(line)
    .map(|cap| {
        (
            Coord2::new(
                cap[1].parse::<i64>().unwrap(),
                cap[2].parse::<i64>().unwrap(),
            ),
            Coord2::new(
                cap[3].parse::<i64>().unwrap(),
                cap[4].parse::<i64>().unwrap(),
            ),
        )
    })
    .expect("Didn't parse")
}

fn determine_beacon_free_range(
    (sensor, beacon): &(Coord2, Coord2),
    y_val: i64,
) -> Option<RangeInclusive<i64>> {
    let beacon_dist = sensor.manhattan_dist(beacon);
    if beacon_dist < (y_val - sensor.y).abs() {
        return None;
    }

    let x_delta = beacon_dist - (y_val - sensor.y).abs();
    Some((sensor.x - x_delta)..=(sensor.x + x_delta))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't have a test for Part 2 because the example doesn't work as cleanly with my assumptions!
    #[test]
    fn check_day15_part1_case1() {
        assert_eq!(
            day15(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            )
            .0,
            "26".to_string()
        )
    }
}

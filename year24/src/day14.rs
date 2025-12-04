// Potential improvements:
//

use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

use grid::coord::Coord2;
use lazy_static::lazy_static;
use regex::Regex;

pub fn day14(input_lines: &[Vec<String>]) -> (String, String) {
    let test = input_lines[0].len() == 12;
    let droids_at = input_lines[0]
        .iter()
        .filter_map(|line| SecurityDroid::from_line(line).quadrant_after_100(test))
        .collect::<Vec<Quadrant>>();
    let mut quad_count = HashMap::new();
    droids_at.iter().for_each(|quadrant| {
        quad_count
            .entry(quadrant)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    });

    let answer1 = quad_count.values().product::<i64>();

    if !test {
        let _droids = input_lines[0]
            .iter()
            .map(|line| SecurityDroid::from_line(line))
            .collect::<Vec<SecurityDroid>>();
        // Reduced range to show the answer without giving it away completely!
        // (6200..6300).for_each(|time| render_at(&droids, time));
    }

    (format!("{}", answer1), format!("{}", 0))
}

fn _render_at(droids: &[SecurityDroid], time: usize) {
    print!("\x1B[2J\x1B[1;1H");
    let test = false;
    let height = if test { 7 } else { 103 };
    let width = if test { 11 } else { 101 };
    let positions = droids
        .iter()
        .map(|droid| {
            let position = droid.start.sum(&droid.velocity.mult(time as i64));
            let mut pos_x = position.x % width;
            if pos_x < 0 {
                pos_x += width;
            }
            let mut pos_y = position.y % height;
            if pos_y < 0 {
                pos_y += height;
            }
            Coord2 { x: pos_x, y: pos_y }
        })
        .collect::<HashSet<Coord2>>();

    println!("TIME: {time}");
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            if positions.contains(&Coord2 { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        });
        println!();
    });
    sleep(Duration::from_millis(100));
}

struct SecurityDroid {
    start: Coord2,
    velocity: Coord2,
}

impl SecurityDroid {
    fn from_line(line: &str) -> Self {
        let line_cap = DROID_DETAILS.captures(line).expect("couldn't regex");
        Self {
            start: Coord2 {
                x: line_cap[1].parse::<i64>().unwrap(),
                y: line_cap[2].parse::<i64>().unwrap(),
            },
            velocity: Coord2 {
                x: line_cap[3].parse::<i64>().unwrap(),
                y: line_cap[4].parse::<i64>().unwrap(),
            },
        }
    }

    fn quadrant_after_100(&self, test: bool) -> Option<Quadrant> {
        let height = if test { 7 } else { 103 };
        let width = if test { 11 } else { 101 };
        let position = self.start.sum(&self.velocity.mult(100));
        let mut pos_x = position.x % width;
        if pos_x < 0 {
            pos_x += width;
        }
        let mut pos_y = position.y % height;
        if pos_y < 0 {
            pos_y += height;
        }

        match (pos_x.cmp(&(width / 2)), pos_y.cmp(&(height / 2))) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(Quadrant::UpperLeft),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(Quadrant::LowerLeft),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(Quadrant::UpperRight),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                Some(Quadrant::LowerRight)
            }
            (std::cmp::Ordering::Equal, _) | (_, std::cmp::Ordering::Equal) => None,
        }
    }
}

lazy_static! {
    pub static ref DROID_DETAILS: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

#[derive(PartialEq, Eq, Hash)]
enum Quadrant {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[cfg(test)]
mod tests {
    use super::day14;
    use crate::utils::load_input;

    #[test]
    fn check_day14_case01() {
        full_test(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3", // INPUT STRING
            "12", // PART 1 RESULT
            "0",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day14(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

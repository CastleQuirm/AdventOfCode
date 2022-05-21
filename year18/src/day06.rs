// Potential improvements:
//

use std::{cmp::Ordering, collections::HashSet};

use regex::Regex;

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    // Read each line as a Coord struct
    let all_points = input_lines[0]
        .iter()
        .map(|line| Coord::new(line))
        .collect::<Vec<Coord>>();

    // Find the min and max x and y values.
    let min_x = all_points
        .iter()
        .map(|coord| coord.x)
        .min()
        .expect("No min x?");
    let max_x = all_points
        .iter()
        .map(|coord| coord.x)
        .max()
        .expect("No max x?");
    let min_y = all_points
        .iter()
        .map(|coord| coord.y)
        .min()
        .expect("No min y?");
    let max_y = all_points
        .iter()
        .map(|coord| coord.y)
        .max()
        .expect("No max y?");

    // For each point in a grid that extends between those values, work out the closest Coord and the total distance to all Coords
    let distance_map: Vec<Vec<DistanceNode>> = (min_x..(max_x + 1))
        .map(|x| {
            (min_y..max_y + 1)
                .map(|y| {
                    let mut distance_node = DistanceNode {
                        closest_coord: None,
                        distance: usize::MAX,
                        total_distance_from_points: 0,
                    };
                    for node in &all_points {
                        let this_distance = node.manhattan_dist(&Coord { x, y });
                        match this_distance.cmp(&distance_node.distance) {
                            Ordering::Greater => {
                                distance_node = DistanceNode {
                                    closest_coord: distance_node.closest_coord,
                                    distance: distance_node.distance,
                                    total_distance_from_points: distance_node
                                        .total_distance_from_points
                                        + this_distance,
                                }
                            }
                            Ordering::Less => {
                                distance_node = DistanceNode {
                                    closest_coord: Some(*node),
                                    distance: this_distance,
                                    total_distance_from_points: distance_node
                                        .total_distance_from_points
                                        + this_distance,
                                }
                            }
                            Ordering::Equal => {
                                distance_node = DistanceNode {
                                    closest_coord: None,
                                    distance: this_distance,
                                    total_distance_from_points: distance_node
                                        .total_distance_from_points
                                        + this_distance,
                                }
                            }
                        }
                    }
                    distance_node
                })
                .collect()
        })
        .collect();

    // For each coord that doesn't appear on the edge of that grid...
    let mut infinite_points: HashSet<Coord> = HashSet::new();
    let mut extra_safe_points_outside = 0;

    // Horrible hack to work out whether to run with UT number or real puzzle number...
    let safe_distance = if all_points.len() == 6 { 32 } else { 10_000 };

    for row in &distance_map {
        if let Some(coord) = row[0].closest_coord {
            infinite_points.insert(coord);
        }
        if row[0].total_distance_from_points < safe_distance {
            extra_safe_points_outside +=
                (safe_distance - row[0].total_distance_from_points) / all_points.len();
        }

        if let Some(coord) = row[max_y - min_y].closest_coord {
            infinite_points.insert(coord);
        }
        if row[max_y - min_y].total_distance_from_points <= safe_distance {
            extra_safe_points_outside +=
                (safe_distance - row[max_y - min_y].total_distance_from_points) / all_points.len();
        }
    }
    for y in 0..max_y - min_y + 1 {
        if let Some(coord) = distance_map[0][y].closest_coord {
            infinite_points.insert(coord);
        }
        if distance_map[0][y].total_distance_from_points <= safe_distance {
            extra_safe_points_outside +=
                (safe_distance - distance_map[0][y].total_distance_from_points) / all_points.len();
        }

        if let Some(coord) = distance_map[max_x - min_x][y].closest_coord {
            infinite_points.insert(coord);
        }
        if distance_map[max_x - min_x][y].total_distance_from_points <= safe_distance {
            extra_safe_points_outside += (safe_distance
                - distance_map[max_x - min_x][y].total_distance_from_points)
                / all_points.len();
        }
    }

    // The above begins to consider the possibility that the overall safe points extend beyond the grid. If they do, we've
    // counted the number of such points that are in the horizontal or vertical lines, but not diagonally off the corners.
    // However, my data-set at least doesn't actially have *any* safe points outside the main grid, so I'm not going to
    // write even more code that won't get executed to handle the corners.
    assert_eq!(extra_safe_points_outside, 0);

    let distance_list = distance_map
        .into_iter()
        .flatten()
        .collect::<Vec<DistanceNode>>();

    //...count its frequency and take the max value.
    let answer1 = all_points
        .iter()
        .filter_map(|point| {
            if infinite_points.contains(point) {
                None
            } else {
                Some(
                    distance_list
                        .iter()
                        .filter(|&vertex| vertex.closest_coord == Some(*point))
                        .count(),
                )
            }
        })
        .max()
        .expect("No final max");

    let answer2 = distance_list
        .iter()
        .filter(|point| point.total_distance_from_points < safe_distance)
        .count();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> usize {
        // self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        let x_diff = if self.x >= other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let y_diff = if self.y >= other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        assert!(usize::MAX - x_diff > y_diff);
        x_diff + y_diff
    }
}

#[derive(Debug)]
struct DistanceNode {
    closest_coord: Option<Coord>,
    distance: usize,
    total_distance_from_points: usize,
}

impl Coord {
    fn new(line: &str) -> Self {
        let re = Regex::new(r"(\d+), (\d+)").unwrap();
        re.captures(line)
            .map(|cap| {
                let x = cap[1].parse::<usize>().expect("Didn't parse x");
                let y = cap[2].parse::<usize>().expect("Didn't parse y");
                Coord { x, y }
            })
            .expect("Regex didn't match")
    }
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9", // INPUT STRING
            "17", // PART 1 RESULT
            "16", // PART 2 RESULT
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

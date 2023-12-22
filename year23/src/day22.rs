// Potential improvements:
//

use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::{coord::Coord2, grid::Grid};

pub fn day22(input_lines: &[Vec<String>]) -> (String, String) {
    // Read the set of bricks, ordered from their lowest starting point.
    // Note the range of x and y coords as we go.
    let (mut max_x, mut max_y) = (0, 0);
    let mut bricks = input_lines[0]
        .iter()
        .map(|line| {
            let brick = Brick::from(line);
            max_x = max_x.max(brick.horizontal_cells.last().expect("non-existent brick").x);
            max_y = max_y.max(brick.horizontal_cells.last().expect("non-existent brick").y);
            brick
        })
        .sorted_by(|a, b| a.starting_bottom.cmp(&b.starting_bottom))
        .collect_vec();

    // Create a top-down grid containing the height of each cell and the index
    // from the sorted list of bricks) of the brick in that top-most cell.
    let mut height_map = Grid::<(i64, Option<usize>)>::initialize(
        (max_x + 1) as usize,
        (max_y + 1) as usize,
        (0i64, None),
    );

    // Drop the bricks one at a time, tracking the single points of support.
    let mut single_supporters = HashSet::new();
    for (brick_ix, brick) in bricks.iter_mut().enumerate() {
        let (lands_on, supporting_bricks) = brick
            .horizontal_cells
            .iter()
            .map(|cell| height_map.get(cell))
            .fold(
                (0, HashSet::<usize>::new()),
                |(acc_h, mut acc_s), (cell_h, cell_c)| {
                    match acc_h.cmp(&cell_h) {
                        std::cmp::Ordering::Less => {
                            // the next cell has given a new limit
                            (
                                cell_h,
                                HashSet::from([cell_c.expect("Must be a brick here")]),
                            )
                        }
                        std::cmp::Ordering::Equal => {
                            // the next cell matches, so just possibly extend the hashset of supports
                            if let Some(cell_support) = cell_c {
                                acc_s.insert(cell_support);
                            }
                            (acc_h, acc_s)
                        }
                        std::cmp::Ordering::Greater => (acc_h, acc_s), // Nothing affected
                    }
                },
            );
        brick.dropped_bottom = lands_on + 1;
        assert!(brick.dropped_bottom <= brick.starting_bottom);
        // brick.supporting_bricks = supporting_bricks;
        match supporting_bricks.len() {
            0 => assert_eq!(lands_on, 0),
            1 => {
                single_supporters.insert(*supporting_bricks.iter().next().unwrap());
            }
            _ => (),
        }
        for cell in &brick.horizontal_cells {
            height_map.set_cell(cell, &(lands_on + brick.height, Some(brick_ix)));
        }
    }

    // Count how many bricks could be individually disintegrated without anything falling.
    let answer1 = bricks.len() - single_supporters.len();
    // Try disintegrating each single supporter one at a time (there's no need to try the others)
    // and see how many bricks land lower than they did before.
    let answer2 = single_supporters
        .iter()
        .map(|target| disintegrate_drop_and_compare(&bricks, *target, max_x, max_y))
        .sum::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn disintegrate_drop_and_compare(
    old_bricks: &[Brick],
    disintegrate_ix: usize,
    max_x: i64,
    max_y: i64,
) -> usize {
    // Create a top-down grid containing the height of each cell and the index
    // from the sorted list of bricks) of the brick in that top-most cell.
    let mut height_map = Grid::<i64>::initialize((max_x + 1) as usize, (max_y + 1) as usize, 0i64);

    let mut bricks = old_bricks.to_owned();

    // Drop the bricks one at a time, skipping the disintegrated one, counting if they land
    // lower than they used to.
    let mut affected_bricks = 0;
    for (brick_ix, brick) in bricks.iter_mut().enumerate() {
        if brick_ix == disintegrate_ix {
            continue;
        }
        let lands_on = brick
            .horizontal_cells
            .iter()
            .map(|cell| height_map.get(cell))
            .fold(0, |acc_h, cell_h| acc_h.max(cell_h));
        brick.dropped_bottom = lands_on + 1;
        assert!(brick.dropped_bottom <= brick.starting_bottom);
        assert!(brick.dropped_bottom <= old_bricks[brick_ix].dropped_bottom);
        if brick.dropped_bottom < old_bricks[brick_ix].dropped_bottom {
            affected_bricks += 1;
        }
        for cell in &brick.horizontal_cells {
            height_map.set_cell(cell, &(lands_on + brick.height));
        }
    }

    affected_bricks
}

#[derive(Clone)]
struct Brick {
    // The cells are always ordered (the last one's indices are not smaller than the first)
    horizontal_cells: Vec<Coord2>,
    height: i64,
    starting_bottom: i64,
    dropped_bottom: i64,
    // supporting_bricks: HashSet<usize>
}

impl From<&String> for Brick {
    fn from(value: &String) -> Self {
        let block_def_regex = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
        let (horizontal_cells, height, starting_bottom) = block_def_regex
            .captures(value)
            .map(|cap| {
                let (x1, y1, z1, x2, y2, z2) = (
                    cap[1].parse::<i64>().unwrap(),
                    cap[2].parse::<i64>().unwrap(),
                    cap[3].parse::<i64>().unwrap(),
                    cap[4].parse::<i64>().unwrap(),
                    cap[5].parse::<i64>().unwrap(),
                    cap[6].parse::<i64>().unwrap(),
                );

                match x1.cmp(&x2) {
                    std::cmp::Ordering::Less => {
                        assert_eq!(y1, y2);
                        assert_eq!(z1, z2);
                        ((x1..=x2).map(|x| Coord2 { x, y: y1 }).collect_vec(), 1, z1)
                    }
                    std::cmp::Ordering::Equal => match y1.cmp(&y2) {
                        std::cmp::Ordering::Less => {
                            assert_eq!(z1, z2);
                            ((y1..=y2).map(|y| Coord2 { x: x1, y }).collect_vec(), 1, z1)
                        }
                        std::cmp::Ordering::Equal => (
                            Vec::from([Coord2 { x: x1, y: y1 }]),
                            (z1 - z2).abs() + 1,
                            z1.min(z2),
                        ),
                        std::cmp::Ordering::Greater => {
                            assert_eq!(z1, z2);
                            ((y2..=y1).map(|y| Coord2 { x: x1, y }).collect_vec(), 1, z1)
                        }
                    },
                    std::cmp::Ordering::Greater => {
                        assert_eq!(y1, y2);
                        assert_eq!(z1, z2);
                        ((x2..=x1).map(|x| Coord2 { x, y: y1 }).collect_vec(), 1, z1)
                    }
                }
            })
            .unwrap();

        Self {
            horizontal_cells,
            height,
            starting_bottom,
            dropped_bottom: starting_bottom,
            // supporting_bricks: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day22;
    use crate::utils::load_input;

    #[test]
    fn check_day22_case01() {
        full_test(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9", // INPUT STRING
            "5", // PART 1 RESULT
            "7", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day22(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

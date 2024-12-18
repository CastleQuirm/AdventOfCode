// Potential improvements:
//

use std::collections::HashSet;

use crate::coord::{Coord2, DELTAS_ORTH_ONLY};

pub fn day18(input_lines: &[Vec<String>]) -> (String, String) {
    let (grid_size, part1_block_count) = if input_lines[0].len() < 100 {
        (6, 12)
    } else {
        (70, 1024)
    };

    let corrupted_cells = input_lines[0]
        .iter()
        .map(|cell| {
            let (x, y) = cell.split_once(',').expect("Bad input line");
            Coord2::new(
                x.parse::<i64>().expect("x NaN"),
                y.parse::<i64>().expect("y NaN"),
            )
        })
        .collect::<Vec<Coord2>>();

    let answer1 = find_route(grid_size, &corrupted_cells, part1_block_count)
        .expect("No solution for part1???");

    // Binary search for part 2!
    let mut lower_bound = part1_block_count;
    let mut upper_bound = corrupted_cells.len();

    while upper_bound > lower_bound + 1 {
        let test_val = lower_bound + (upper_bound - lower_bound) / 2;
        if find_route(grid_size, &corrupted_cells, test_val).is_some() {
            lower_bound = test_val;
        } else {
            upper_bound = test_val;
        }
    }

    // Note that while lower_bound represents the blocks that still allow a path while upper bound is
    // when we have blocks preventing a path, the blocks used are everything up to BUT NOT INCLUDING
    // the given bound value. So we actually want upper_bound - 1 for the index of the coordinates of
    // the block that's the partitioner, which is the same as lower_bound
    let answer2 = corrupted_cells[lower_bound];
    (
        format!("{}", answer1),
        format!("{},{}", answer2.x, answer2.y),
    )
}

fn find_route(grid_size: i64, corrupted_cells: &[Coord2], active_blocks: usize) -> Option<i32> {
    // Mini-Dijkstra - more like just water filling simulation
    let walls = corrupted_cells[0..active_blocks]
        .iter()
        .collect::<HashSet<_>>();
    let target_space = Coord2::new(grid_size, grid_size);
    let mut visited_cells = HashSet::<Coord2>::new();
    let mut next_step_cells = HashSet::<Coord2>::from([Coord2::new(0, 0)]);
    let mut steps = 0;
    while !next_step_cells.is_empty() {
        let mut future_step_cells = HashSet::<Coord2>::new();
        steps += 1;

        for next in next_step_cells {
            for delta in DELTAS_ORTH_ONLY.iter() {
                let candidate = next.sum(delta);
                if candidate.x >= 0
                    && candidate.y >= 0
                    && candidate.x <= grid_size
                    && candidate.y <= grid_size
                    && !walls.contains(&candidate)
                    && !visited_cells.contains(&candidate)
                {
                    if candidate == target_space {
                        return Some(steps);
                    }
                    future_step_cells.insert(candidate);
                }
            }
            visited_cells.insert(next);
        }

        next_step_cells = future_step_cells;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::day18;
    use crate::utils::load_input;

    #[test]
    fn check_day18_case01() {
        full_test(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0", // INPUT STRING
            "22",  // PART 1 RESULT
            "6,1", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day18(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

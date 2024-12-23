// Potential improvements:
//

use std::collections::HashMap;

use crate::{
    coord::{Coord2, DELTAS_ORTH_ONLY},
    grid::Grid,
};

pub fn day20(input_lines: &[Vec<String>]) -> (String, String) {
    let maze = Grid::<MazeCells>::from_input(&input_lines[0]);
    let start = maze
        .find_single_element(&MazeCells::Start)
        .expect("No start");
    let end = maze.find_single_element(&MazeCells::End).expect("No end");

    // Dijkstra a path but expecting it to be a single route throughout
    let mut main_course = HashMap::new();
    let mut next_step = Some((start, 0));
    let mut ordered_path = Vec::new();

    while let Some((next_loc, time)) = next_step {
        // For every cell except the first and last we expect: 2 walls, 1 space already visited, 1 space not visited
        let mut walls = 0;
        let mut already_been = 0;
        let mut unvisited = 0;
        let mut next_next_loc = None;
        DELTAS_ORTH_ONLY.iter().for_each(|delta| {
            let adjacent_pos = next_loc.sum(delta);
            match maze.peek(&adjacent_pos) {
                MazeCells::Wall => walls += 1,
                MazeCells::Track | MazeCells::Start | MazeCells::End => {
                    if main_course.contains_key(&adjacent_pos) {
                        already_been += 1;
                    } else {
                        unvisited += 1;
                        assert_eq!(next_next_loc, None);
                        next_next_loc = Some((adjacent_pos, time + 1));
                    }
                }
            }
        });
        if next_loc == start || next_loc == end {
            assert_eq!(walls, 3);
        } else {
            assert_eq!(walls, 2);
        }
        if next_loc != end {
            assert!(next_next_loc.is_some());
        } else {
            assert!(next_next_loc.is_none());
        }
        if next_loc != start {
            assert_eq!(already_been, 1);
        } else {
            assert_eq!(already_been, 0);
        }

        main_course.insert(next_loc, time);
        ordered_path.push(next_loc);
        next_step = next_next_loc;
    }

    // For every step along the road, see if 2 spaces away we can find somewhere at least 102 (or 40 in the case of
    // the test) ahead.
    let required_jump = if maze.width() > 100 { 102 } else { 40 };
    let answer1 = main_course
        .iter()
        .map(|(loc, time)| {
            // assume for simplicity there's no corner shortcuts.
            DELTAS_ORTH_ONLY
                .iter()
                .filter(|d| {
                    main_course
                        .get(&loc.sum(&d.mult(2)))
                        .is_some_and(|&new_time| {
                            // let improvement = new_time - *time;
                            // if improvement > 0 {
                            //     println!("found a skip of {improvement} from {:?} with d {:?}", loc, d);
                            // }
                            new_time >= *time + required_jump
                        })
                })
                .count()
        })
        .sum::<usize>();

    let required_jump = if maze.width() > 100 { 100 } else { 72 };
    let answer2 = count_shortcuts(&ordered_path, 20, required_jump);
    (format!("{}", answer1), format!("{}", answer2))
}

fn count_shortcuts(path: &[Coord2], cheat_size: i64, saving_required: usize) -> usize {
    (0..(path.len() - saving_required))
        .map(|shortcut_start_ix| {
            (shortcut_start_ix + saving_required..path.len())
                .filter(|&shortcut_end_ix| {
                    let start_coord = path[shortcut_start_ix];
                    let end_coord = path[shortcut_end_ix];
                    let shortcut_cost = end_coord.manhattan_dist(&start_coord);
                    shortcut_cost <= cheat_size
                        && (shortcut_end_ix - shortcut_start_ix - shortcut_cost as usize)
                            >= saving_required
                })
                .count()
        })
        .sum::<usize>()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum MazeCells {
    Track,
    Wall,
    Start,
    End,
}

impl From<char> for MazeCells {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Track,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day20;
    use crate::utils::load_input;

    #[test]
    fn check_day20_case01() {
        full_test(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############", // INPUT STRING
            "3",  // PART 1 RESULT // arbitrary choice of save of 38 or better
            "29", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day20(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// Potential improvements:
//

use std::collections::{BinaryHeap, HashSet};

use grid::{coord::Coord2, directions::CompassDirection, Grid};

pub fn day16(input_lines: &[Vec<String>]) -> (String, String) {
    let maze = Grid::<MazeMap>::from_input(&input_lines[0]);

    let exit = maze
        .find_single_element(&MazeMap::Exit)
        .expect("No way out!");
    let start = maze
        .find_single_element(&MazeMap::Start)
        .expect("Gotta take part to win");

    let mut visited_locations = HashSet::<ReindeerLoc>::new();
    let mut explorable_locations = BinaryHeap::<Route>::new();
    explorable_locations.push(Route {
        reindeer: ReindeerLoc {
            location: start,
            facing: CompassDirection::East,
        },
        cost: 0,
        visited_locs: HashSet::from([start]),
    });
    let mut answer1 = u64::MAX;
    let mut all_steps_on_optimals = HashSet::new();

    // Give me a D, give me an I, give me a J, give me a K, give me a S, give me a T, give me an R, give me an A!
    while let Some(next_explore) = explorable_locations.pop() {
        if next_explore.cost > answer1 {
            break;
        }

        if next_explore.reindeer.location == exit {
            all_steps_on_optimals = all_steps_on_optimals
                .union(&next_explore.visited_locs)
                .cloned()
                .collect::<HashSet<Coord2>>();
            answer1 = next_explore.cost;
        }

        // Three potential candidates: rotate 90 degrees in either direction, or move forwards.
        let mut possible_moves = Vec::from([
            Route {
                reindeer: ReindeerLoc {
                    location: next_explore.reindeer.location,
                    facing: next_explore
                        .reindeer
                        .facing
                        .rotate(&grid::directions::Rotation::Left),
                },
                cost: next_explore.cost + 1000,
                visited_locs: next_explore.visited_locs.clone(),
            },
            Route {
                reindeer: ReindeerLoc {
                    location: next_explore.reindeer.location,
                    facing: next_explore
                        .reindeer
                        .facing
                        .rotate(&grid::directions::Rotation::Right),
                },
                cost: next_explore.cost + 1000,
                visited_locs: next_explore.visited_locs.clone(),
            },
        ]);

        let forward_move = next_explore
            .reindeer
            .location
            .compass_sum(&next_explore.reindeer.facing);
        if maze.peek(&forward_move) != &MazeMap::Wall {
            let mut visited_locs_plus_new = next_explore.visited_locs.clone();
            visited_locs_plus_new.insert(forward_move);

            possible_moves.push(Route {
                reindeer: ReindeerLoc {
                    location: next_explore
                        .reindeer
                        .location
                        .compass_sum(&next_explore.reindeer.facing),
                    facing: next_explore.reindeer.facing,
                },
                cost: next_explore.cost + 1,
                visited_locs: visited_locs_plus_new,
            })
        }

        possible_moves
            .iter()
            .filter(|next_step| {
                // This is actually pretty inefficient in simple Dijkstra - we're going to repeat each latter part of
                // a path for evey equal earlier path that can get there. But that helps us for part 2!
                // We could still be improved by digging out any that do match and combining the routes, but that
                // seems like an annoyance from a heap.
                !visited_locations.contains(&next_step.reindeer)
            })
            .for_each(|next_step| explorable_locations.push(next_step.clone()));
        visited_locations.insert(next_explore.reindeer);
    }

    let answer2 = all_steps_on_optimals.len();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct ReindeerLoc {
    location: Coord2,
    facing: CompassDirection,
}

#[derive(Eq, PartialEq, Clone)]
struct Route {
    reindeer: ReindeerLoc,
    cost: u64,
    visited_locs: HashSet<Coord2>,
}

impl Ord for Route {
    // Reverse ordering to use a min-heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MazeMap {
    Wall,
    Space,
    Start,
    Exit,
}

impl From<char> for MazeMap {
    fn from(value: char) -> Self {
        match value {
            '#' => MazeMap::Wall,
            '.' => MazeMap::Space,
            'S' => MazeMap::Start,
            'E' => MazeMap::Exit,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day16;
    use crate::utils::load_input;

    #[test]
    fn check_day16_case01() {
        full_test(
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############", // INPUT STRING
            "7036", // PART 1 RESULT
            "45",   // PART 2 RESULT
        )
    }

    #[test]
    fn check_day16_case02() {
        full_test(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################", // INPUT STRING
            "11048", // PART 1 RESULT
            "64",    // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day16(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

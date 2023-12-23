// Potential improvements:
//

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{
    coord::Coord2,
    directions::{
        CompassDirection::{self, East, North, South, West},
        Rotation,
    },
    grid::Grid,
};

pub fn day23(input_lines: &[Vec<String>]) -> (String, String) {
    // Get the maze. It comes with a border!
    let maze = Grid::<MazeSpace>::from_input(&input_lines[0]);

    // Find the start and end. The example and my input both have the start at (1, 0) and the end
    // at (width - 1, height), but the problem only states they're in the top and bottom row so
    // we'll be polite and find them from that info.
    let find_space_in_row = |y: usize| Coord2 {
        x: maze.grid[y]
            .iter()
            .find_position(|s| **s == MazeSpace::Space)
            .and_then(|(pos, _)| TryInto::<i64>::try_into(pos).ok())
            .expect("Error finding entrance"),
        y: TryInto::<i64>::try_into(y).expect("Bad y index"),
    };
    let start = find_space_in_row(0);
    let end = find_space_in_row(maze.grid.len() - 1);

    // Explore the maze, building up a series of paths.
    let paths = explore(&maze, &start, &end);

    // Explore the set of paths, looking for the longest continuous route without repeats.
    let answer1 = longest_route(&paths, &start, &end, true);
    let answer2 = longest_route(&paths, &start, &end, false);
    (format!("{}", answer1), format!("{}", answer2))
}

fn explore(maze: &Grid<MazeSpace>, start: &Coord2, end: &Coord2) -> HashMap<Coord2, PathsFrom> {
    let mut established_paths: HashMap<Coord2, PathsFrom> = HashMap::new();
    let mut to_explore: Vec<(Coord2, CompassDirection)> = Vec::from([(*start, South)]);
    while let Some((rally_point, start_dir)) = to_explore.pop() {
        let mut current_loc = rally_point;
        let mut possible_next_dirs = Vec::from([start_dir]);
        let mut steps_walked = 0;
        let mut current_dir = possible_next_dirs[0];
        let mut difficult_terrain = false;

        while possible_next_dirs.len() == 1 {
            current_dir = possible_next_dirs[0];
            current_loc = current_loc.compass_sum(&current_dir);
            steps_walked += 1;
            if current_loc == *end || current_loc == *start {
                // If we've reached the exit, we should stop this particular exploration.
                break;
            }
            match maze.get(&current_loc) {
                MazeSpace::Space => {
                    // Check where we can go next (without reversing)
                    possible_next_dirs = [Rotation::Straight, Rotation::Left, Rotation::Right]
                        .into_iter()
                        .filter_map(|rot| {
                            let new_dir = current_dir.rotate(&rot);
                            if maze.get(&current_loc.compass_sum(&new_dir)) != MazeSpace::Wall {
                                Some(new_dir)
                            } else {
                                None
                            }
                        })
                        .collect_vec();
                }
                MazeSpace::OneWay { allowed_dir } => {
                    // This is always just a straigh-on path so we don't need to change
                    // possible_next_dirs
                    assert!(allowed_dir == current_dir || allowed_dir == current_dir.opposite());
                    // If this isn't the direction of the arrow, then this trail is difficult.
                    if allowed_dir == current_dir.opposite() {
                        difficult_terrain = true;
                    }
                }
                MazeSpace::Wall => unreachable!("Shouldn't be walking into walls!"),
            }
        }

        if !possible_next_dirs.is_empty() {
            // We've established a path.
            let known_trail = established_paths
                .entry(rally_point)
                .or_default()
                .routes
                .insert(
                    start_dir,
                    Trail {
                        destination: current_loc,
                        distance: steps_walked,
                        incoming_dir: current_dir,
                        difficult_terrain,
                    },
                );
            assert!(known_trail.is_none());

            // If there was more than one possible direction left and we haven't already explored this site, this is a junction to explore on from.
            if possible_next_dirs.len() > 1 && !established_paths.contains_key(&current_loc) {
                for dir in possible_next_dirs {
                    to_explore.push((current_loc, dir));
                }
                to_explore.push((current_loc, current_dir.opposite()))
            }
        }
    }
    established_paths
}

fn longest_route(
    paths: &HashMap<Coord2, PathsFrom>,
    start: &Coord2,
    end: &Coord2,
    block_difficult_terrain: bool,
) -> usize {
    let mut longest_walk = 0;

    let mut ongoing_walks: Vec<Walk> = Vec::from([Walk {
        current_loc: *start,
        visited_junctions: HashSet::from([*start]),
        distance: 0,
        latest_dir: South,
    }]);
    while let Some(walk) = ongoing_walks.pop() {
        paths
            .get(&walk.current_loc)
            .expect("Unknown junction!")
            .routes
            .iter()
            .for_each(|(outgoing_dir, trail)| {
                if *outgoing_dir == walk.latest_dir.opposite() {
                    // Don't want to just turn around
                    return;
                }
                if block_difficult_terrain && trail.difficult_terrain {
                    // This trail has difficult terrain, which isn't allowed at the moment
                    return;
                }
                let next_junction = trail.destination;
                let new_distance = walk.distance + trail.distance;
                if next_junction == *end {
                    // Path reached the end, check the overall walk distance
                    longest_walk = longest_walk.max(new_distance);
                } else if !walk.visited_junctions.contains(&next_junction) {
                    let mut visited_junctions = walk.visited_junctions.clone();
                    visited_junctions.insert(next_junction);
                    ongoing_walks.push(Walk {
                        current_loc: next_junction,
                        visited_junctions,
                        distance: new_distance,
                        latest_dir: trail.incoming_dir,
                    })
                }
            });
    }

    longest_walk
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct PathsFrom {
    routes: HashMap<CompassDirection, Trail>,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Trail {
    destination: Coord2,
    distance: usize,
    incoming_dir: CompassDirection,
    difficult_terrain: bool,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Walk {
    current_loc: Coord2,
    visited_junctions: HashSet<Coord2>,
    distance: usize,
    latest_dir: CompassDirection,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum MazeSpace {
    Space,
    OneWay { allowed_dir: CompassDirection },
    Wall,
}

impl From<char> for MazeSpace {
    fn from(value: char) -> Self {
        match value {
            '.' => MazeSpace::Space,
            '>' => MazeSpace::OneWay { allowed_dir: East },
            '<' => MazeSpace::OneWay { allowed_dir: West },
            'v' => MazeSpace::OneWay { allowed_dir: South },
            '^' => MazeSpace::OneWay { allowed_dir: North },
            // '.' | '<' | '>' | '^' | 'v' => MazeSpace::Space,
            '#' => MazeSpace::Wall,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day23;
    use crate::utils::load_input;

    #[test]
    fn check_day23_case01() {
        full_test(
            "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#", // INPUT STRING
            "94",  // PART 1 RESULT
            "154", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day23(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// Potential improvements:
//

use std::collections::{HashMap, HashSet};

pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    let directions = input_lines[0][0].chars().collect::<Vec<char>>();

    let pathmaps = input_lines[1]
        .iter()
        .map(|line| {
            let (source, dest) = line.split_once(" = ").expect("Bad line format");
            let (left, right) = dest
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .expect("Bad line format");
            (
                source.to_owned(),
                Node {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            )
        })
        .collect::<HashMap<String, Node>>();

    let mut current_location = "AAA".to_string();
    let mut num_steps = 0;

    while current_location != *"ZZZ" {
        let next_direction = directions[num_steps % directions.len()];
        let node = pathmaps
            .get(&current_location)
            .expect("No map from where we are!");
        current_location = match next_direction {
            'L' => node.left.clone(),
            'R' => node.right.clone(),
            _ => panic!(),
        };
        num_steps += 1;
    }
    let answer1 = num_steps;

    // Create a map of all the next hops needed to get from any Z to the next Z from any point in the pathing.
    let mut z_to_z_steps: HashMap<(String, usize), (String, usize)> = HashMap::new();
    for start_z in pathmaps.keys().filter(|loc| loc.ends_with('Z')) {
        for start_index in 0..directions.len() {
            num_steps = 0;
            current_location = start_z.clone();
            while !current_location.ends_with('Z') || num_steps == 0 {
                // TODO factor out
                let next_direction = directions[num_steps % directions.len()];
                let node = pathmaps
                    .get(&current_location)
                    .expect("No map from where we are!");
                current_location = match next_direction {
                    'L' => node.left.clone(),
                    'R' => node.right.clone(),
                    _ => panic!(),
                };
                num_steps += 1;
            }
            z_to_z_steps.insert(
                (start_z.clone(), start_index),
                (current_location.clone(), num_steps),
            );
        }
    }

    // Move each ghost to its first Z location, counting how many steps it's taken.
    let mut ghosts = pathmaps
        .keys()
        .filter_map(|loc| {
            if loc.ends_with('A') {
                num_steps = 0;
                current_location = loc.clone();
                while !current_location.ends_with('Z') {
                    // TODO factor out
                    let next_direction = directions[num_steps % directions.len()];
                    let node = pathmaps
                        .get(&current_location)
                        .expect("No map from where we are!");
                    current_location = match next_direction {
                        'L' => node.left.clone(),
                        'R' => node.right.clone(),
                        _ => panic!(),
                    };
                    num_steps += 1;
                }

                Some((current_location.clone(), num_steps))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // If not every ghost has moved the same distance, move the one that's moved the least.
    while ghosts
        .iter()
        .map(|(_, steps)| *steps)
        .collect::<HashSet<usize>>()
        .len()
        > 1
    {
        let least_moved_ghost = ghosts
            .iter_mut()
            .min_by(|g1, g2| g1.1.cmp(&g2.1))
            .expect("Spoooooooky");
        let ghost_lookup = (
            least_moved_ghost.0.clone(),
            least_moved_ghost.1 % directions.len(),
        );
        let next_jump = z_to_z_steps
            .get(&ghost_lookup)
            .expect("Lost track of the ghost");
        *least_moved_ghost = (next_jump.0.clone(), least_moved_ghost.1 + next_jump.1)
    }
    let answer2 = ghosts[0].1;
    (format!("{}", answer1), format!("{}", answer2))
}

struct Node {
    left: String,
    right: String,
}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)", // INPUT STRING
            "2", // PART 1 RESULT
            "2", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day08_case02() {
        full_test(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)", // INPUT STRING
            "6", // PART 1 RESULT
            "6", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day08_case03() {
        full_test(
            "LR

AAA = (11B, XXX)
11B = (XXX, ZZZ)
ZZZ = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)", // INPUT STRING
            "2", // PART 1 RESULT
            "6", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day08(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

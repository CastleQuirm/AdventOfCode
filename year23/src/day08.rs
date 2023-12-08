// Potential improvements:
//

use std::collections::HashMap;

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
    let answer2 = 0;
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
            "0", // PART 2 RESULT
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
            "0", // PART 2 RESULT
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

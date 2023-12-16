// Potential improvements:
//

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn day15(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0][0].split(',').map(hash).sum::<u64>();

    let step_sections_regex = Regex::new(r"([a-z]+)([=-])(\d?)").unwrap();
    let instructions = step_sections_regex
        .captures_iter(&input_lines[0][0])
        .map(|cap| {
            let action = match &cap[2] {
                "=" => Action::Insert {
                    focal_length: cap[3].parse::<u64>().unwrap(),
                },
                "-" => {
                    assert_eq!(&cap[3], "");
                    Action::Remove
                }
                _ => unreachable!(),
            };
            Instruction {
                label: cap[1].to_string(),
                action,
            }
        })
        .collect::<Vec<Instruction>>();

    let mut box_contents: HashMap<u64, Vec<(String, u64)>> = HashMap::new();
    for instruction in instructions {
        let frame = box_contents.entry(hash(&instruction.label)).or_default();
        let existing_lens = frame
            .iter()
            .find_position(|lens| lens.0 == instruction.label)
            .map(|(position, _lens)| position);
        match instruction.action {
            Action::Remove => {
                if let Some(removable_lens) = existing_lens {
                    frame.remove(removable_lens);
                }
            }
            Action::Insert { focal_length } => {
                if let Some(replaceable_lens) = existing_lens {
                    frame.remove(replaceable_lens);
                    frame.insert(replaceable_lens, (instruction.label.clone(), focal_length));
                } else {
                    frame.push((instruction.label.clone(), focal_length))
                }
            }
        }
    }

    let answer2 = box_contents
        .iter()
        .map(|(box_number, lenses)| {
            (1 + box_number)
                * lenses
                    .iter()
                    .enumerate()
                    .map(|(pos, (_label, focal_length))| (1 + pos as u64) * focal_length)
                    .sum::<u64>()
        })
        .sum::<u64>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn hash(step: &str) -> u64 {
    step.chars().fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

struct Instruction {
    label: String,
    action: Action,
}

enum Action {
    Remove,
    Insert { focal_length: u64 },
}

#[cfg(test)]
mod tests {
    use super::day15;
    use crate::utils::load_input;

    #[test]
    fn check_day15_case01() {
        full_test(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", // INPUT STRING
            "1320",                                                // PART 1 RESULT
            "145",                                                 // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day15(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

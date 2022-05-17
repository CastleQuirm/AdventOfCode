// Potential improvements:
//

use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
    string::ParseError,
};

use regex::Regex;

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let mut outstanding_dependencies = input_lines[0]
        .iter()
        .map(|line| line.parse::<Dependency>().expect("Failed to parse line"))
        .collect::<Vec<Dependency>>();

    let mut available_steps = BTreeSet::new();
    let mut completed_steps = "".to_string();

    // Create the initial options of available steps.  This is every step that is a dependency but isn't a dependant.
    let (dependency_steps, dependant_steps) = outstanding_dependencies
        .iter()
        .map(|rule| (rule.dependency, rule.dependant))
        .unzip::<char, char, HashSet<char>, HashSet<char>>();
    for first_step in dependency_steps.difference(&dependant_steps) {
        available_steps.insert(*first_step);
    }

    // Start working through the list of work - do the first available step, then work out if any new steps are now available
    let step_count = dependency_steps.union(&dependant_steps).count();
    while completed_steps.len() < step_count {
        let next_step = *available_steps.iter().next().expect("No available steps!");
        available_steps.remove(&next_step);
        completed_steps.push(next_step);

        let possibly_unblocked: Vec<char> = outstanding_dependencies
            .iter()
            .filter_map(|rule| {
                if rule.dependency == next_step {
                    Some(rule.dependant)
                } else {
                    None
                }
            })
            .collect();

        outstanding_dependencies = outstanding_dependencies
            .into_iter()
            .filter(|rule| rule.dependency != next_step)
            .collect();

        for unblocked_step in possibly_unblocked.iter().filter(|step| {
            outstanding_dependencies
                .iter()
                .all(|rule| rule.dependant != **step)
        }) {
            available_steps.insert(*unblocked_step);
        }
    }

    let answer2 = 0;
    (completed_steps.to_string(), format!("{}", answer2))
}

struct Dependency {
    dependency: char,
    dependant: char,
}

impl FromStr for Dependency {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
        re.captures(line)
            .map(|cap| {
                let dependency = cap[1].parse::<char>().expect("Didn't parse dependency");
                let dependant = cap[2].parse::<char>().expect("Didn't parse dependant");
                Ok(Self {
                    dependency,
                    dependant,
                })
            })
            .expect("Regex didn't match")
    }
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.", // INPUT STRING
            "CABDFE", // PART 1 RESULT
            "0",      // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

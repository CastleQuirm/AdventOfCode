// Potential improvements:
//

use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
    string::ParseError,
};

use regex::Regex;

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let outstanding_dependencies = input_lines[0]
        .iter()
        .map(|line| line.parse::<Dependency>().expect("Failed to parse line"))
        .collect::<Vec<Dependency>>();

    let (answer1, _) = build_sled(&outstanding_dependencies, 1);
    let (_, answer2) = build_sled(&outstanding_dependencies, 5);

    (answer1, format!("{}", answer2))
}

fn build_sled(initial_dependencies: &[Dependency], worker_count: usize) -> (String, usize) {
    // Create our starting state, including a collection of un-tasked workers.
    let mut outstanding_dependencies: Vec<Dependency> = initial_dependencies.to_vec();
    let mut available_steps: BTreeSet<char> = BTreeSet::new();
    let mut completed_steps = "".to_string();

    let mut workers: Vec<Worker> = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        workers.push(Worker::new());
    }

    let mut time = 0;

    // Create the initial options of available steps.  This is every step that is a dependency but isn't a dependant.
    let (dependency_steps, dependant_steps) = outstanding_dependencies
        .iter()
        .map(|rule| (rule.dependency, rule.dependant))
        .unzip::<char, char, HashSet<char>, HashSet<char>>();
    for first_step in dependency_steps.difference(&dependant_steps) {
        available_steps.insert(*first_step);
    }

    // Check how many steps we have to do, so we know when the sled is built.
    let step_count = dependency_steps.union(&dependant_steps).count();

    // Loop the following: while there are steps left to do...
    // - if a worker is free and a task is availbale, give them the task
    // - else, advance time to the next point a worker is free.
    while completed_steps.len() < step_count {
        let free_worker = workers.iter().enumerate().find(|(_, w)| w.task.is_none());
        let next_step = available_steps.iter().next();

        if free_worker.is_some() && next_step.is_some() {
            let next_step = *next_step.unwrap();
            available_steps.remove(&next_step);

            let free_worker_ix = free_worker.unwrap().0;
            let time_needed = (next_step as usize) - ('A' as usize) + 61;

            workers[free_worker_ix] = Worker {
                task: Some(next_step),
                remaining: time_needed,
            }
        } else {
            let time_to_next_complete = workers
                .iter()
                .filter_map(|w| {
                    if w.task.is_some() {
                        Some(w.remaining)
                    } else {
                        None
                    }
                })
                .min()
                .expect("No min?");
            let mut new_workers = workers.clone();
            time += time_to_next_complete;
            for (i, worker) in workers.iter().enumerate() {
                if worker.task.is_none() {
                    continue;
                }
                if worker.remaining > time_to_next_complete {
                    new_workers[i] = Worker {
                        task: worker.task,
                        remaining: worker.remaining - time_to_next_complete,
                    };
                } else {
                    new_workers[i] = Worker::new();
                    let completed_step = worker.task.unwrap();
                    completed_steps.push(completed_step);

                    let possibly_unblocked: Vec<char> = outstanding_dependencies
                        .iter()
                        .filter_map(|rule| {
                            if rule.dependency == completed_step {
                                Some(rule.dependant)
                            } else {
                                None
                            }
                        })
                        .collect();

                    outstanding_dependencies = outstanding_dependencies
                        .into_iter()
                        .filter(|rule| rule.dependency != completed_step)
                        .collect();

                    for unblocked_step in possibly_unblocked.iter().filter(|step| {
                        outstanding_dependencies
                            .iter()
                            .all(|rule| rule.dependant != **step)
                    }) {
                        available_steps.insert(*unblocked_step);
                    }
                }
            }
            workers = new_workers;
        }
    }

    (completed_steps, time)
}

#[derive(Clone)]
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

#[derive(Clone, Debug)]
struct Worker {
    task: Option<char>,
    remaining: usize,
}

impl Worker {
    fn new() -> Self {
        Worker {
            task: None,
            remaining: 0,
        }
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
            "15",     // PART 2 RESULT
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

// Potential improvements:
//

use chrono::{prelude::*, Duration};
use regex::Regex;
use std::{collections::HashMap, ops::Add};

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let mut full_rota: HashMap<NaiveDate, Night> = HashMap::new();
    let re = Regex::new(r"\[(.+)\] (.+)").unwrap();
    for line in &input_lines[0] {
        // find date
        let (full_date, log) = re
            .captures(line)
            .map(|cap| {
                let regex_date = cap[1].parse::<String>().expect("Didn't parse date/time");
                let date = DateTime::parse_from_str(
                    &format!("{}:00 +00:00", regex_date),
                    "%Y-%m-%d %H:%M:%S %z",
                )
                .expect("Couldn't read the date and time");
                let log = cap[2].parse::<String>().expect("Didn't parse log");

                (date, log)
            })
            .expect("Regex didn't match");

        // Get the date this really applies to
        let day = full_date.date_naive();
        let hour = full_date.hour();
        let day = if hour == 23 {
            day.add(Duration::days(1))
        } else {
            day
        };

        // find or create Night
        let night = full_rota.entry(day).or_insert_with(Night::new);

        // update Night with statement.
        night.update(full_date.minute(), &log);
    }

    // DEBUG VALIDATION
    assert!(full_rota.values().all(|night| night.validate_complete()));

    // Find the single guard who spent the most time asleep
    let mut time_asleep: HashMap<i32, u32> = HashMap::new();
    for night in full_rota.values() {
        let minutes_napping = time_asleep
            .entry(night.guard.expect("No guard known this night"))
            .or_insert(0);
        *minutes_napping += night.time_asleep();
    }
    let sleepy_guard = time_asleep
        .iter()
        .max_by_key(|(_guard, &minutes_napping)| minutes_napping)
        .unwrap()
        .0;

    // Create sleep graphs for every guard (we only need them for sleepy_guard in Part 1, but we need them all for Part 2)
    let mut guard_sleep_collections: HashMap<i32, Vec<i32>> = HashMap::new();
    for mut night in full_rota.into_values() {
        night.build_sleep_graph();
        let sleep_count = guard_sleep_collections
            .entry(night.guard.expect("No guard?"))
            .or_insert_with(|| vec![0; 60]);
        for (j, count) in sleep_count.iter_mut().enumerate().take(60) {
            *count += night.sleep_graph[j];
        }
    }

    let sleepiest_minutes = guard_sleep_collections
        .iter()
        .map(|(guard, sleep)| {
            (
                *guard,
                sleep
                    .iter()
                    .enumerate()
                    .max_by_key(|(_minute, &number)| number)
                    .expect("Couldn't get a max"),
            )
        })
        .collect::<HashMap<i32, (usize, &i32)>>();

    // Find the single minute that sleepy guard was most frequently asleep.
    let sleepy_guard_sleepy_minute = sleepiest_minutes
        .get(sleepy_guard)
        .expect("no sleepy guard?")
        .0;

    // Find the single (guard, minute) pair that's best.
    let part2_choice = sleepiest_minutes
        .iter()
        .max_by_key(|(_guard, (_minute, sleep))| sleep)
        .expect("No best?");

    let answer1 = sleepy_guard * sleepy_guard_sleepy_minute as i32;
    let answer2 = part2_choice.0 * part2_choice.1 .0 as i32;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug)]
struct Night {
    guard: Option<i32>,
    falls_asleep: Vec<u32>,
    wakes_up: Vec<u32>,
    sleep_graph: [i32; 60],
}

impl Night {
    fn new() -> Self {
        Night {
            guard: None,
            falls_asleep: Vec::new(),
            wakes_up: Vec::new(),
            sleep_graph: [0; 60],
        }
    }

    fn validate_complete(&self) -> bool {
        self.guard.is_some() && self.falls_asleep.len() == self.wakes_up.len()
    }

    fn build_sleep_graph(&mut self) {
        for falls_asleep_min in &self.falls_asleep {
            for i in *falls_asleep_min..60 {
                self.sleep_graph[i as usize] += 1;
            }
        }

        for wakes_up_min in &self.wakes_up {
            for i in *wakes_up_min..60 {
                assert_ne!(self.sleep_graph[i as usize], 0);
                self.sleep_graph[i as usize] -= 1;
            }
        }
    }

    fn time_asleep(&self) -> u32 {
        self.wakes_up.iter().sum::<u32>() - self.falls_asleep.iter().sum::<u32>()
    }

    fn update(&mut self, minute: u32, log: &str) {
        let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        match guard_re
            .captures(log)
            .map(|cap| cap[1].parse::<i32>().expect("Didn't parse guard number"))
        {
            Some(guard_number) => self.guard = Some(guard_number),
            None => {
                if log == "falls asleep" {
                    self.falls_asleep.push(minute);
                } else if log == "wakes up" {
                    self.wakes_up.push(minute);
                } else {
                    panic!("Unrecognised text! {}", log);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up", // INPUT STRING
            "240",  // PART 1 RESULT
            "4455", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

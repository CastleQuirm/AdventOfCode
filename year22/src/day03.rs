use std::{collections::HashSet, str::Lines};

use itertools::{Chunk, Itertools};

pub fn day03(input_lines: &str) -> (String, String) {
    let answer1 = input_lines
        .lines()
        .map(find_duplicated_priority)
        .sum::<u32>();
    let answer2 = input_lines
        .lines()
        .chunks(3)
        .into_iter()
        .map(find_badge_priority)
        .sum::<u32>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn find_duplicated_priority(line: &str) -> u32 {
    item_priority(
        **line
            .chars()
            .take(line.len() / 2)
            .collect::<HashSet<_>>()
            .intersection(&line.chars().skip(line.len() / 2).collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .first()
            .expect("Just asserted there was one!"),
    )
}

fn find_badge_priority(group: Chunk<Lines>) -> u32 {
    item_priority(
        group
            .collect::<Vec<_>>()
            .iter()
            .map(|elf| elf.chars().collect::<HashSet<_>>())
            .reduce(|candidates, elf| {
                candidates
                    .intersection(&elf)
                    .cloned()
                    .collect::<HashSet<_>>()
            })
            .expect("Must be a collection")
            .drain()
            .next()
            .expect("Must be a badge"),
    )
}

fn item_priority(c: char) -> u32 {
    c.to_digit(36).expect("must be a digit") - 9 + 26 * u32::from(c.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(day03("").0, "0".to_string())
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(day03("").1, "0".to_string())
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(
            day03(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            ("157".to_string(), "70".to_string())
        )
    }
}

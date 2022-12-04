use std::{ops::RangeInclusive, str::FromStr, string::ParseError};

use once_cell::sync::OnceCell;
use regex::Regex;

pub fn day04(input_lines: &str) -> (String, String) {
    let elf_duos = input_lines
        .lines()
        .map(|line| line.parse::<WorkOrders>().expect("Couldn't parse"));
    let answer1 = elf_duos
        .clone()
        .filter(|work| work.elf_fully_overlapped())
        .count();
    let answer2 = elf_duos
        .filter(|work| work.elf_partially_overlapped())
        .count();
    (format!("{}", answer1), format!("{}", answer2))
}

struct WorkOrders {
    elf1: RangeInclusive<u64>,
    elf2: RangeInclusive<u64>,
}

static RE: OnceCell<Regex> = OnceCell::new();

impl FromStr for WorkOrders {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RE
            .get_or_init(|| Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap())
            .captures(s)
            .map(|cap| Self {
                elf1: cap[1].parse::<u64>().unwrap()..=cap[2].parse::<u64>().unwrap(),
                elf2: cap[3].parse::<u64>().unwrap()..=cap[4].parse::<u64>().unwrap(),
            })
            .expect("Didn't parse"))
    }
}

impl WorkOrders {
    fn elf_fully_overlapped(&self) -> bool {
        (self.elf1.contains(self.elf2.start()) && self.elf1.contains(self.elf2.end()))
            || (self.elf2.contains(self.elf1.start()) && self.elf2.contains(self.elf1.end()))
    }

    fn elf_partially_overlapped(&self) -> bool {
        self.elf1.contains(self.elf2.start()) || self.elf2.contains(self.elf1.start())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(
            day04(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            ),
            ("2".to_string(), "4".to_string())
        )
    }
}

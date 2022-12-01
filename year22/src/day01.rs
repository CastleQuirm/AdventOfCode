use itertools::Itertools;

use crate::utils::split_input_on_line_breaks;

pub fn day01(input_lines: &str) -> (String, String) {
    let input_set = split_input_on_line_breaks(input_lines);

    let elves_calories = input_set.iter().map(|elf_list| {
        elf_list
            .iter()
            .map(|item| item.parse::<u64>().expect("Oh no"))
            .sum::<u64>()
    });
    let elves_calories: Vec<_> = elves_calories.sorted().rev().collect();

    let answer1 = elves_calories.first().expect("No elves?");
    let answer2 = elves_calories[0..=2].iter().sum::<u64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(
            day01(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            ),
            ("24000".to_string(), "45000".to_string())
        )
    }
}

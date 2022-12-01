use crate::utils::split_input_by_blocks;

pub fn day01(input_lines: &str) -> (String, String) {
    let mut elves = split_input_by_blocks::<u64>(input_lines, elf_calories);
    elves.sort();
    let answer1 = elves.last().unwrap();
    let answer2 = elves[elves.len() - 3..].iter().sum::<u64>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn elf_calories(lines: &[&str]) -> u64 {
    lines
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .sum::<u64>()
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

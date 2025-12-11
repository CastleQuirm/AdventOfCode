// Potential improvements:
//

use itertools::Itertools;

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0]
        .iter()
        .map(|line| solve_line(line))
        .sum::<usize>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn solve_line(line: &str) -> usize {
    let mut chunks = line.split_ascii_whitespace();
    let target = chunks
        .next()
        .unwrap()
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap();
    let light_count = target.len();
    let target = u64::from_str_radix(&target.replace('.', "0").replace('#', "1"), 2).unwrap();
    let mut buttons = chunks.collect::<Vec<&str>>();
    buttons.pop();
    let buttons = buttons
        .iter()
        .map(|button| {
            let buttons = button
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(',')
                .map(|num| num.parse::<u32>().unwrap());
            buttons.fold(0, |acc, e| acc + 2u64.pow(light_count as u32 - e - 1))
        })
        .collect::<Vec<u64>>();

    // Find the smallest subset of buttons that give the target.
    (1..=buttons.len())
        .find(|presses| {
            buttons
                .iter()
                .cloned()
                .combinations(*presses)
                .map(|c| c.into_iter().reduce(|acc, e| acc ^ e).unwrap())
                .any(|val| val == target)
        })
        .expect("No valid combination of buttons!")
}

#[cfg(test)]
mod tests {
    use super::day10;
    use crate::utils::load_input;

    #[test]
    fn check_day10_case01() {
        full_test(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", // INPUT STRING
            "7", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day10(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

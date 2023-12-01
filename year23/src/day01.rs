// Potential improvements:
//

use std::collections::HashMap;

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0].iter().map(convert_line_to_num).sum::<u32>();

    let digit_strings_and_vals = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let answer2 = input_lines[0]
        .iter()
        .map(|line| {            
            let mut first_digit = 0;
            let mut first_digit_index = usize::MAX;
            let mut last_digit = 0;
            let mut last_digit_index = usize::MIN;
            
            for (text, value) in &digit_strings_and_vals {
                if let Some(left_index) = line.find(text) {
                    if left_index < first_digit_index {
                        first_digit = *value;
                        first_digit_index = left_index;
                    }
                }
                // The rfind would be dubious if any key string was wholly contained in another...but none are.
                if let Some(right_index) = line.rfind(text) {
                    if right_index >= last_digit_index {
                        last_digit = *value;
                        last_digit_index = right_index; 
                    }
                }
            }
            10 * first_digit + last_digit
        })
        .sum::<u32>();

    (format!("{}", answer1), format!("{}", answer2))
}

fn convert_line_to_num(line: &String) -> u32 {
    let characters = line.chars();
    let tens = characters.clone().find_map(|c| c.to_digit(10)).unwrap_or(0);
    let ones = characters.rev().find_map(|c| c.to_digit(10)).unwrap_or(0);
    10 * tens + ones
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet", // INPUT STRING
            "142", // PART 1 RESULT
            "142", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day01_case02() {
        full_test(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
            "209", // PART 1 RESULT
            "281", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day01(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

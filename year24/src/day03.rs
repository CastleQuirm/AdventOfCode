use regex::Regex;

// Potential improvements:
//

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let mut active = true;
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mult_regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)|do(.?.?.?)\(\)").unwrap();
    input_lines[0].iter().for_each(|line| {
        for cap in mult_regex.captures_iter(line) {
            match &cap[0] {
                "do()" => active = true,
                "don't()" => active = false,
                _ => {
                    let sum = cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap();
                    answer1 += sum;
                    if active {
                        answer2 += sum;
                    }
                }
            }
        }
    });

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", // INPUT STRING
            "161", // PART 1 RESULT
            "161", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day03_case02() {
        full_test(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", // INPUT STRING
            "161", // PART 1 RESULT
            "48",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

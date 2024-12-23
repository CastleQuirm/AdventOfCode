// Potential improvements:
//

pub fn day22(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0]
        .iter()
        .rev()
        .map(|line| {
            let mut number = line.parse::<u64>().expect("couldn't parse");
            (0..2000).for_each(|_| {
                number = compute_next_secret(number);
            });
            number
        })
        .sum::<u64>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn compute_next_secret(current_secret: u64) -> u64 {
    ((current_secret ^ ((current_secret * 64) % 16777216))
        ^ ((current_secret ^ ((current_secret * 64) % 16777216)) / 32))
        ^ ((((current_secret ^ ((current_secret * 64) % 16777216))
            ^ ((current_secret ^ ((current_secret * 64) % 16777216)) / 32))
            * 2048)
            % 16777216)
}

#[cfg(test)]
mod tests {
    use super::day22;
    use crate::utils::load_input;

    #[test]
    fn check_day22_case01() {
        full_test(
            "1
10
100
2024", // INPUT STRING
            "37327623", // PART 1 RESULT
            "23",       // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day22(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

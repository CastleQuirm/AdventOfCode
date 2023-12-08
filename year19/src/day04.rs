// Potential improvements:
//

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let mut input_ends = input_lines[0][0].split('-');
    let lower_limit = input_ends.next().expect("Didn't read lower limit").parse::<usize>().expect("Couldn't parse");
    let higher_limit = input_ends.next().expect("Didn't read higher limit").parse::<usize>().expect("Couldn't parse");
    assert!(input_ends.next().is_none());

    let answer_1 = (lower_limit..=higher_limit).filter(valid_password).count();

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn valid_password(num: &usize) -> bool {
    let password = 

}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
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

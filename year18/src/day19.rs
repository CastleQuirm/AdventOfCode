// Potential improvements:
//

use crate::utils::OpCode;

pub fn day19(_input_lines: &[Vec<String>]) -> (String, String) {
    // Read instructions

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day19;
    use crate::utils::load_input;

    #[test]
    fn check_day19_case01() {
        full_test(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5", // INPUT STRING
            "6", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day19(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

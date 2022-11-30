pub fn day03(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
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
        assert_eq!(day03(""), ("0".to_string(), "0".to_string()))
    }
}

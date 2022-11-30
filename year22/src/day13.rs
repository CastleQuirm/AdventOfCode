pub fn day13(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day13_part1_case1() {
        assert_eq!(day13("").0, "0".to_string())
    }

    #[test]
    fn check_day13_part2_case1() {
        assert_eq!(day13("").1, "0".to_string())
    }

    #[test]
    fn check_day13_both_case1() {
        assert_eq!(day13(""), ("0".to_string(), "0".to_string()))
    }
}

pub fn day08(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(day08("").0, "0".to_string())
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(day08("").1, "0".to_string())
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(day08(""), ("0".to_string(), "0".to_string()))
    }
}

pub fn day17(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day17_part1_case1() {
        assert_eq!(day17("").0, "0".to_string())
    }

    #[test]
    fn check_day17_part2_case1() {
        assert_eq!(day17("").1, "0".to_string())
    }

    #[test]
    fn check_day17_both_case1() {
        assert_eq!(day17(""), ("0".to_string(), "0".to_string()))
    }
}

pub fn day23(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day23_part1_case1() {
        assert_eq!(day23("").0, "0".to_string())
    }

    #[test]
    fn check_day23_part2_case1() {
        assert_eq!(day23("").1, "0".to_string())
    }

    #[test]
    fn check_day23_both_case1() {
        assert_eq!(day23(""), ("0".to_string(), "0".to_string()))
    }
}

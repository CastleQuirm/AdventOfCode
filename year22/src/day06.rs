use std::collections::HashSet;

pub fn day06(input_lines: &str) -> (String, String) {
    (
        format!("{}", find_window_end(input_lines, 4)),
        format!("{}", find_window_end(input_lines, 14)),
    )
}

fn find_window_end(input: &str, size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(size)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == size)
        .expect("No start?")
        + size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(
            day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            ("7".to_string(), "19".to_string())
        );
        assert_eq!(
            day06("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            ("5".to_string(), "23".to_string())
        );
        assert_eq!(
            day06("nppdvjthqldpwncqszvftbrmjlhg"),
            ("6".to_string(), "23".to_string())
        );
        assert_eq!(
            day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            ("10".to_string(), "29".to_string())
        );
        assert_eq!(
            day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            ("11".to_string(), "26".to_string())
        );
    }
}

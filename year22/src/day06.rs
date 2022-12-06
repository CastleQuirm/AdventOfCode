use std::collections::HashSet;

pub fn day06(input_lines: &str) -> (String, String) {
    let answer1 = find_window_end(input_lines, 4);
    let answer2 = find_window_end(input_lines, 14);
    (format!("{}", answer1), format!("{}", answer2))
}

fn find_window_end(input: &str, size: usize) -> usize {
    let stream = input.chars().collect::<Vec<_>>();
    let mut windows = stream.windows(size);
    // Find the text for the start of stream.
    let start_of_stream = windows
        .find(|&w| w.iter().collect::<HashSet<_>>().len() == size)
        .expect("No start?")
        .iter()
        .collect::<String>();
    // Find that text in the file.
    input.find(&start_of_stream).expect("Where did it start?") + size
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

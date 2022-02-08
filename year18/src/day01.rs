// Potential improvements:
//

use std::collections::HashSet;

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0].iter().fold(0, |i, val| {
        i + val.parse::<i32>().expect("Couldn't read number")
    });

    let mut visited_frequencies = HashSet::new();
    let mut answer2 = 0;
    'outer: loop {
        for line in input_lines[0].iter() {
            answer2 += line.parse::<i32>().expect("Couldn't read number");
            if !visited_frequencies.insert(answer2) {
                break 'outer;
            }
        }
    }
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "+1
-2
+3
+1", // INPUT STRING
            "3", // PART 1 RESULT
            "2", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day01(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

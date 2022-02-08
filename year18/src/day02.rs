// Potential improvements:
//

use std::collections::{HashMap, HashSet};

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let doubles_and_triples = input_lines[0].iter().fold((0, 0), |counts, line| {
        let duplicate_letters = count_duplicates(line);
        (
            counts.0 + duplicate_letters.0,
            counts.1 + duplicate_letters.1,
        )
    });

    let answer1 = doubles_and_triples.0 * doubles_and_triples.1;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn count_duplicates(line: &str) -> (u64, u64) {
    let mut letter_count: HashMap<char, u64> = HashMap::new();
    line.chars().for_each(|c| {
        let count = letter_count.entry(c).or_insert(0);
        *count += 1;
    });
    let overall_counts = letter_count.values().collect::<HashSet<&u64>>();
    let contains_double = if overall_counts.contains(&&2) { 1 } else { 0 };
    let contains_triple = if overall_counts.contains(&&3) { 1 } else { 0 };
    (contains_double, contains_triple)
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab", // INPUT STRING
            "12", // PART 1 RESULT
            "0",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

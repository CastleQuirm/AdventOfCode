// Potential improvements:
//

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    (day02_part1(input_lines), day02_part2(input_lines))
}

fn day02_part1(input_lines: &[Vec<String>]) -> String {
    let doubles_and_triples = input_lines[0].iter().fold((0, 0), |counts, line| {
        let duplicate_letters = count_duplicates(line);
        (
            counts.0 + duplicate_letters.0,
            counts.1 + duplicate_letters.1,
        )
    });

    format!("{}", doubles_and_triples.0 * doubles_and_triples.1)
}

fn day02_part2(input_lines: &[Vec<String>]) -> String {
    let word_len = input_lines[0][0].len();
    let answer2 = input_lines[0].iter().combinations(2).find_map(|pair| {
        let intersection = string_intersection(pair[0], pair[1]);
        if intersection.len() == word_len - 1 {
            Some(intersection)
        } else {
            None
        }
    });
    answer2.expect("Couldn't find an answer")
}

fn count_duplicates(line: &str) -> (u64, u64) {
    let mut letter_count: HashMap<char, u64> = HashMap::new();
    line.chars().for_each(|c| {
        let count = letter_count.entry(c).or_insert(0);
        *count += 1;
    });
    let overall_counts = letter_count.values().collect::<HashSet<&u64>>();
    let contains_double = u64::from(overall_counts.contains(&&2));
    let contains_triple = u64::from(overall_counts.contains(&&3));
    (contains_double, contains_triple)
}

// Considered doing the specific thing for the puzzle (return Option<String> with the value if it's the one we want)
// but decided to instead have a nicer function and have the caller do the smarts.
fn string_intersection(string_a: &str, string_b: &str) -> String {
    let mut intersection = "".to_string();
    let chars_a = string_a.chars();
    let mut chars_b = string_b.chars();
    for char_1 in chars_a {
        let char_2 = chars_b.next().expect("Word 2 was shorter than word 1");
        if char_1 == char_2 {
            intersection.push(char_1)
        }
    }
    assert_eq!(chars_b.next(), None);
    intersection
}

#[cfg(test)]
mod tests {
    use super::day02_part1;
    use super::day02_part2;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        assert_eq!(
            day02_part1(&load_input(
                "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"
            )),
            "12"
        );
    }

    #[test]
    fn check_day02_case02() {
        assert_eq!(
            day02_part2(&load_input(
                "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            )),
            "fgij"
        );
    }
}

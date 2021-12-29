// Potential improvements:
// 1: Implement count_characters_with_and() without using the Intersect crate?  (first check suggested Union was slower than my own method; may be same for Intersect?)
// 2: Improve the name of "function"
// 3: As with Day 4, make all days use the same input.

extern crate array_tool;
use array_tool::vec::Intersect;

pub fn day6(input_lines: &[String]) -> (u64, u64) {
    (
        answer_with_func(input_lines, count_characters_with_or),
        answer_with_func(input_lines, count_characters_with_and),
    )
}

fn answer_with_func(input_lines: &[String], function: fn(&str) -> u64) -> u64 {
    input_lines[0]
        .split("\n\n")
        .map(|group| function(group))
        .sum()
}

fn count_characters_with_or(group: &str) -> u64 {
    lowercase_alphabet()
        .iter()
        .filter(|&&letter| group.to_string().contains(letter))
        .count() as u64
}

// Tried making a version of the or function using the Union trait, similarly to Intersect, but it's possibly slower?  Too much noise to really tell.
// fn count_characters_with_or_crate(group: &str) -> u64 {
//     group
//         .to_string()
//         .lines()
//         .fold(vec![], |union, line| {
//             union.union(line.chars().collect::<Vec<char>>())
//         })
//         .len() as u64
// }

fn count_characters_with_and(group: &str) -> u64 {
    group
        .to_string()
        .lines()
        .fold(lowercase_alphabet(), |intersection, line| {
            intersection.intersect(line.chars().collect::<Vec<char>>())
        })
        .len() as u64
}

fn lowercase_alphabet() -> Vec<char> {
    (b'a'..=b'z').map(|c| c as char).collect()
}

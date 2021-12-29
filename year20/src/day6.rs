// Potential improvements:
// 1: Implement count_characters_with_and() without using the Intersect crate?  (first check suggested Union was slower than my own method; may be same for Intersect?)
// 2: Improve the name of "function".

extern crate array_tool;
use array_tool::vec::Intersect;

pub fn day6(input_lines: &[String]) -> (u64, u64) {
    (
        answer_with_func(input_lines, count_characters_with_or),
        answer_with_func(input_lines, count_characters_with_and),
    )
}

fn answer_with_func(input_lines: &[String], function: fn(&[String]) -> u64) -> u64 {
    let mut group_answers: Vec<Vec<String>> = Vec::new();
    let mut responses_in_group: Vec<String> = Vec::new();
    for line in input_lines {
        if line.is_empty() {
            group_answers.push(responses_in_group.clone());
            responses_in_group = Vec::new();
        } else {
            responses_in_group.push(line.clone());
        }
    }
    group_answers.push(responses_in_group);

    group_answers.iter()
        .map(|group| function(group))
        .sum()
}

fn count_characters_with_or(group: &[String]) -> u64 {
    lowercase_alphabet()
        .iter()
        .filter(|&&letter| group.iter().any(|respondant| respondant.contains(letter)))
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

fn count_characters_with_and(group: &[String]) -> u64 {
    group.iter()
        .fold(lowercase_alphabet(), |intersection, line| {
            intersection.intersect(line.chars().collect::<Vec<char>>())
        })
        .len() as u64
}

fn lowercase_alphabet() -> Vec<char> {
    (b'a'..=b'z').map(|c| c as char).collect()
}

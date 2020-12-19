// Potential improvements:
// 1: Use the actual input!  I've cheated and avoided questions over escaping inverted commas by deleting them manually in the input file
// 2: Use an actual language parser, see https://en.wikipedia.org/wiki/CYK_algorithm
// 3: Make the permute_elements into a trait-generic function and available in a library; it'd be useful in other puzzles.  However, need to (a) remove the string concatanation element (and supply that elsewhere) and work around another problem I hit
// 4: Tidy up the check_language function to be a bit less if/if else/elsey.
// 5: Maybe do something cleverer with the check_language in general (although, really, just do point 2 instead).

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn day19(input_lines: &[String]) -> (u64, u64) {
    // Parse the input as a map of rules and a vec of candidates.
    let (rules, candidates) = parse_input(input_lines);

    // To get the answer to part 2: observe that the format for a valid string is simply 8 11, and these are our two changed rules.
    // Call 42's string match X and 31's string match Y.
    // 8 is 1+ copies of X, and 11 is 1+ copies of X followed by the same number of copies of Y.
    // X and Y are both 8 character strings (although have 128 valid options each). They're probably a partition of the 8-binary space but haven't checked.
    // Combined, this means a valid word in the language:
    // - 1) must be 0mod8 characters long, and at least 24 characters total
    // - 2) when broken into 8 character substrings, must have the first two blocks be Xs and the last block be a Y
    // - 3) If there are any more substrings within that, must be all Xs then all Ys (no mixing) and must be at least half Xs.
    // Part 1 is then a simplified subset of this: the string must be 24 characters exactly and be of the form XXY.

    // Original code (albeit refactored)
    // Iteratively expand the 0 rule
    // let valid_0_strings = candidate_substrings(&rules, 0);
    // let part1_answer = HashSet::from_iter(candidates.clone())
    //     .intersection(&valid_0_strings)
    //     .count() as u64;

    let x_string = candidate_substrings(&rules, 42);
    let y_string = candidate_substrings(&rules, 31);

    let part1_answer = candidates
        .iter()
        .filter(|candidate| {
            candidate.len() == 24 && check_language(&x_string, &y_string, candidate)
        })
        .count() as u64;
    let part2_answer = candidates
        .iter()
        .filter(|candidate| {
            candidate.len() >= 24 && check_language(&x_string, &y_string, candidate)
        })
        .count() as u64;

    (part1_answer, part2_answer)
}

fn check_language(x: &HashSet<String>, y: &HashSet<String>, candidate: &str) -> bool {
    if candidate.len() % 8 != 0 {
        false
    } else {
        let block_count = candidate.len() / 8;
        let mut x_allowed = true;
        (0..block_count).all(|i| {
            let block = &candidate[8 * i..8 * (i + 1)];
            if 2 * i <= block_count {
                // X is mandatory until the half-way mark
                x.contains(block)
            } else if i == block_count - 1 {
                // The last element must be Y
                y.contains(block)
            } else if x_allowed && x.contains(block) {
                // We're past half-way. If X is allowed and it is an X, carry on
                true
            } else if y.contains(block) {
                // We're past half-way and X isn't allowed, or it isn't an X, but it is a Y
                x_allowed = false;
                true
            } else {
                // It isn't Y, and it either isn't X or isn't allowed to be.  Fail.
                false
            }
        })
    }
}

fn candidate_substrings(rules: &HashMap<usize, Rule>, index: usize) -> HashSet<String> {
    let mut valid_strings: HashSet<String> = HashSet::from_iter(
        rules
            .get(&index)
            .expect("Didn't find rule")
            .constructs
            .clone(),
    );
    while not_just_letters(&valid_strings) {
        valid_strings = HashSet::from_iter(substitute_rules(&valid_strings, &rules).clone());
    }
    valid_strings
        .iter()
        .map(|string| string.replace(" ", ""))
        .collect::<HashSet<String>>()
}

fn substitute_rules(valid_strings: &HashSet<String>, rules: &HashMap<usize, Rule>) -> Vec<String> {
    valid_strings
        .iter()
        .map(|candidate| substitute_candidate(candidate, rules))
        .flatten()
        .collect::<Vec<String>>()
}

fn substitute_candidate(string: &str, rules: &HashMap<usize, Rule>) -> Vec<String> {
    permute_elements(
        string
            .split(' ')
            .map(|elt| replacement_string(elt, rules))
            .collect(),
    )
}

fn permute_elements(layered: Vec<Vec<String>>) -> Vec<String> {
    layered
        .iter()
        .fold(vec!["".to_string()], |result: Vec<String>, elt| {
            result
                .iter()
                .map(|pre| {
                    let mapped_pre = elt
                        .iter()
                        .map(|post| {
                            let mut added_space = pre.to_string();
                            if !added_space.is_empty() {
                                added_space.push_str(" ");
                            }
                            added_space.push_str(post);
                            added_space
                        })
                        .collect::<Vec<String>>();
                    mapped_pre
                })
                .flatten()
                .collect::<Vec<String>>()
        })
}

fn replacement_string(elt: &str, rules: &HashMap<usize, Rule>) -> Vec<String> {
    match elt.parse::<usize>() {
        Ok(num) => rules.get(&num).unwrap().constructs.to_vec(),
        Err(_) => vec![elt.to_string()],
    }
}

fn not_just_letters(rule_set: &HashSet<String>) -> bool {
    rule_set.iter().any(|candidate| {
        candidate
            .chars()
            .any(|c| c.to_string().parse::<u64>().is_ok())
    })
}

fn parse_input(input_lines: &[String]) -> (HashMap<usize, Rule>, Vec<String>) {
    (
        input_lines
            .iter()
            .filter(|line| line.contains(':'))
            .map(|line| Rule::new(line))
            .fold(HashMap::new(), |mut map, rule| {
                map.insert(rule.index, rule);
                map
            }),
        input_lines
            .iter()
            .filter(|line| !line.contains(':') && !line.is_empty())
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>(),
    )
}

#[derive(Clone)]
struct Rule {
    index: usize,
    constructs: Vec<String>,
}
impl Rule {
    fn new(line: &str) -> Rule {
        let index = line
            .split(": ")
            .next()
            .expect("Couldn't find an index for the rule")
            .parse::<usize>()
            .unwrap();
        let constructs = line
            .split(": ")
            .nth(1)
            .expect("Couldn't find anything after the rule")
            .split(" | ")
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        Rule { index, constructs }
    }
}

#[cfg(test)]
mod tests {
    use super::day19;

    //     #[test]
    //     fn day19_example_part1() {
    //         // Cheat input: (a) get rid of the inverted commas just in case
    //         let input =
    // "0: 4 1 5
    // 1: 2 3 | 3 2
    // 2: 4 4 | 5 5
    // 3: 4 5 | 5 4
    // 4: a
    // 5: b

    // ababbb
    // bababa
    // abbbab
    // aaabbb
    // aaaabbb"
    //             .lines()
    //             .map(std::string::ToString::to_string)
    //             .collect::<Vec<String>>();
    //         assert_eq!(day19(&input).0, 2);
    //     }

    //     #[test]
    //     fn day19_example_part2() {
    //         // Cheat input: (a) get rid of the inverted commas just in case
    //         let input = "42: 9 14 | 10 1
    // 9: 14 27 | 1 26
    // 10: 23 14 | 28 1
    // 1: a
    // 11: 42 31
    // 5: 1 14 | 15 1
    // 19: 14 1 | 14 14
    // 12: 24 14 | 19 1
    // 16: 15 1 | 14 14
    // 31: 14 17 | 1 13
    // 6: 14 14 | 1 14
    // 2: 1 24 | 14 4
    // 0: 8 11
    // 13: 14 3 | 1 12
    // 15: 1 | 14
    // 17: 14 2 | 1 7
    // 23: 25 1 | 22 14
    // 28: 16 1
    // 4: 1 1
    // 20: 14 14 | 1 15
    // 3: 5 14 | 16 1
    // 27: 1 6 | 14 18
    // 14: b
    // 21: 14 1 | 1 14
    // 25: 1 1 | 1 14
    // 22: 14 14
    // 8: 42
    // 26: 14 22 | 1 20
    // 18: 15 15
    // 7: 14 5 | 1 21
    // 24: 14 1

    // abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
    // bbabbbbaabaabba
    // babbbbaabbbbbabbbbbbaabaaabaaa
    // aaabbbbbbaaaabaababaabababbabaaabbababababaaa
    // bbbbbbbaaaabbbbaaabbabaaa
    // bbbababbbbaaaaaaaabbababaaababaabab
    // ababaaaaaabaaab
    // ababaaaaabbbaba
    // baabbaaaabbaaaababbaababb
    // abbbbabbbbaaaababbbbbbaaaababb
    // aaaaabbaabaaaaababaa
    // aaaabbaaaabbaaa
    // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
    // babaaabbbaaabaababbaabababaaab
    // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
    //             .lines()
    //             .map(std::string::ToString::to_string)
    //             .collect::<Vec<String>>();
    //         assert_eq!(day19(&input), (3, 12));
    //     }
}

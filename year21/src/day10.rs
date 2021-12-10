// Potential improvements:
// 1. The separate runs over the scores iterator, filter-mapping as a partition, feels inelegant.
// 2. The median function has been used twice now (here and day 07).  Should commonize (or get a crate!)
// 3. Should consider renaming part1, part2 as meaningful to the problem.

pub fn day10(input_lines: &[String]) -> (u64, u64) {
    let scores = input_lines.iter().map(|line| syntax_score(line));

    let part1 = scores
        .clone()
        .filter_map(|score| match score {
            ScoreType::Syntax { score: val } => Some(val),
            ScoreType::Autocomplete { score: _ } => None,
        })
        .sum();

    let mut autocomplete_scores = scores
        .filter_map(|score| match score {
            ScoreType::Syntax { score: _ } => None,
            ScoreType::Autocomplete { score: val } => Some(val),
        })
        .collect::<Vec<u64>>();
    autocomplete_scores.sort_unstable();

    // The problem setter actually says it'll be an odd number of entries, so we could skip straight to the else branch
    // but maybe better to just keep the function.
    let part2 = if autocomplete_scores.len() % 2 == 0 {
        let double_median = autocomplete_scores[autocomplete_scores.len() / 2 - 1]
            + autocomplete_scores[autocomplete_scores.len() / 2];
        if double_median % 2 != 0 {
            println!("Median got rounded down!");
        }
        double_median / 2
    } else {
        autocomplete_scores[(autocomplete_scores.len() - 1) / 2]
    };

    (part1, part2)
}

fn syntax_score(line: &str) -> ScoreType {
    let mut open_stack: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '{' | '[' | '<' => open_stack.push(c),
            ')' => {
                if open_stack.pop() != Some('(') {
                    return ScoreType::Syntax { score: 3 };
                }
            }
            ']' => {
                if open_stack.pop() != Some('[') {
                    return ScoreType::Syntax { score: 57 };
                }
            }
            '}' => {
                if open_stack.pop() != Some('{') {
                    return ScoreType::Syntax { score: 1197 };
                }
            }
            '>' => {
                if open_stack.pop() != Some('<') {
                    return ScoreType::Syntax { score: 25137 };
                }
            }
            _ => panic!("Unrecognised char!"),
        }
    }

    // If we've reached here, the line is incomplete and can be autocompleted.
    ScoreType::Autocomplete {
        score: open_stack.iter().rfold(0, |val, c| {
            let new_val: u64 = match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("What character is this? How did it get here?"),
            };
            val * 5 + new_val
        }),
    }
}

#[derive(PartialEq, Eq)]
enum ScoreType {
    Syntax { score: u64 },
    Autocomplete { score: u64 },
}

#[cfg(test)]
mod tests {
    use super::day10;

    #[test]
    fn check_day10() {
        let input_lines = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day10(&input_lines), (26397, 288957));
    }
}

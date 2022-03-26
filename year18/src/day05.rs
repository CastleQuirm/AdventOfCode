// Potential improvements:
//

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let first_reduced = reduce(&input_lines[0][0], None);

    let alphabet = (b'a'..b'z' + 1).map(|i| i as char).collect::<Vec<_>>();

    let answer1 = first_reduced.len();
    let answer2 = alphabet
        .iter()
        .map(|&c| reduce(&first_reduced, Some(c)).len())
        .min()
        .expect("No minimum?");
    (format!("{}", answer1), format!("{}", answer2))
}

fn reduce(start_string: &str, always_remove: Option<char>) -> String {
    let mut left_string: Vec<char> = Vec::new();
    let mut left_compare: Option<char> = None;
    let mut right_compare: Option<char> = None;
    let mut right_string: Vec<char> = start_string.chars().collect::<Vec<char>>();

    while !right_string.is_empty() {
        while right_string.first().map(|c| c.to_ascii_lowercase()) == always_remove {
            right_string = right_string.split_off(1);
        }

        if right_string.is_empty() {
            break;
        }

        if left_compare.is_none() {
            left_compare = left_string.pop();
        } else {
            left_string.push(left_compare.unwrap());
            left_compare = None;
        }

        if left_compare.is_none() {
            left_compare = right_compare;
            right_compare = None;
        }

        if right_compare.is_none() {
            right_compare = right_string.first().cloned();
            right_string = right_string.split_off(1);
        }

        if left_compare.is_some()
            && right_compare.is_some()
            && (go_compare(left_compare, right_compare) || go_compare(right_compare, left_compare))
        {
            left_compare = None;
            right_compare = None;
        }
    }

    if let Some(left_char) = left_compare {
        left_string.push(left_char);
    }
    if let Some(right_char) = right_compare {
        left_string.push(right_char);
    }

    left_string.iter().collect::<String>()
}

fn go_compare(first: Option<char>, second: Option<char>) -> bool {
    first.unwrap().is_uppercase()
        && second.unwrap().is_lowercase()
        && first.map(|c| c.to_ascii_lowercase()) == second
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
            "dabAcCaCBAcCcaDA", // INPUT STRING
            "10",               // PART 1 RESULT
            "4",                // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day05(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

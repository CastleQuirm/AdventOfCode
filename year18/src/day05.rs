// Potential improvements:
//

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    // Reduce the string as fully as possible.
    let first_reduced = reduce(&input_lines[0][0], None);

    // Create a Vec of the alphabet.
    let alphabet = (b'a'..b'z' + 1).map(|i| i as char).collect::<Vec<_>>();

    // The first answer is just the length of the reduced string.
    let answer1 = first_reduced.len();

    // The second answer is the length of the shortest possible reduced string after removing all instances of any one
    // character.  We can optimise by just reducing the already basic reduced string, since anything that cancelled out there
    // will be cancelled out again (or would automatically be removed fully).
    let answer2 = alphabet
        .iter()
        .map(|&c| reduce(&first_reduced, Some(c)).len())
        .min()
        .expect("No minimum?");
    (format!("{}", answer1), format!("{}", answer2))
}

fn reduce(start_string: &str, always_remove: Option<char>) -> String {
    // Process the string by moving through it left to right.
    // left_string will represent characters we've moved past (unless we need to start pulling in from it due to deletions).
    // right_string are characters we haven't reached yet.
    // left_compare and right_compare are the characters we're looking at for a comparison.
    // The overall string is the concatination, in order, of left_string -> left_compare -> right_compare -> right_string
    let mut left_string: Vec<char> = Vec::new();
    let mut left_compare: Option<char> = None;
    let mut right_compare: Option<char> = None;
    let mut right_string: Vec<char> = start_string.chars().collect::<Vec<char>>();

    // Loop while we still have new characters to look at.
    while !right_string.is_empty() {
        // Get rid of any sequence of characters we're about to hit matching our 'always_remove' option.
        while right_string.first().map(|c| c.to_ascii_lowercase()) == always_remove {
            right_string = right_string.split_off(1);
        }

        // If that finished the string, bail.
        if right_string.is_empty() {
            break;
        }

        // The following empty block is how I originally wrote this.  It's shorter code than the active version below
        // but is much harder to reason about and review, so less maintainable.  I could write comments explaining the logic
        // but thought a more logical implementation was the better choice.
        {
            // if left_compare.is_none() {
            //     left_compare = left_string.pop();
            // } else {
            //     left_string.push(left_compare.unwrap());
            //     left_compare = None;
            // }
            //
            // if left_compare.is_none() {
            //     left_compare = right_compare;
            //     right_compare = None;
            // }
            //
            // if right_compare.is_none() {
            //     right_compare = right_string.first().cloned();
            //     right_string = right_string.split_off(1);
            // }
        }

        // This is the new version of the processing logic. We match on the current state of what's in our 'hands' (the *_compare Options)
        // whether there's anything in the left string to pull in from, and whether there's enough in the right string to make a comparison.
        match (
            !left_string.is_empty(),
            left_compare.is_some(),
            right_compare.is_some(),
            right_string.len() >= 2,
        ) {
            (false, false, false, false) => {
                // We've got nothing in hand, nothing already processed and exactly one character in the right string (if it was empty,
                // we'd have bailed out earlier in this function). Move the string to left_string to say we're done with it.
                left_string = right_string;
                right_string = Vec::new();
            }
            (false, false, false, true) => {
                // We've got nothing processed - we've just started, or everything we've looked at so far has been eliminated.
                // The right string has multiple characters, so there's still work to do.
                // Grab the first two characters to do a comparison with.
                left_compare = right_string.first().cloned();
                right_compare = right_string.get(1).cloned();
                right_string = right_string.split_off(2);
            }
            (true, false, false, _) => {
                // We've got nothing in hand, but have previously processed letters - we must have just done a comparison that
                // eliminated the pair.  Pull in the closest letter on either side.
                left_compare = left_string.pop();
                right_compare = right_string.first().cloned();
                right_string = right_string.split_off(1);
            }
            (_, true, true, _) => {
                // We've got characters in both hands: a previous comparison failed. Shuffle ahead one character.
                left_string.push(left_compare.unwrap());
                left_compare = right_compare;
                right_compare = right_string.first().cloned();
                right_string = right_string.split_off(1);
            }
            _ => unreachable!(), // Every valid scenario should be handled above.
        }

        // Check if the two held characters are the same, and if so, just delete them both.
        if go_compare(left_compare, right_compare) {
            left_compare = None;
            right_compare = None;
        }
    }

    // Finish moving the characters on to the left string so the entire string is in one piece.
    if let Some(left_char) = left_compare {
        left_string.push(left_char);
    }
    if let Some(right_char) = right_compare {
        left_string.push(right_char);
    }

    // Return the reduced string.
    left_string.iter().collect::<String>()
}

// Check if we have two characters, and if so if they match and have opposite cases.
fn go_compare(first: Option<char>, second: Option<char>) -> bool {
    match (first, second) {
        (Some(first), Some(second)) => {
            (first.is_uppercase() && second.is_lowercase() && first.to_ascii_lowercase() == second)
                || (first.is_lowercase()
                    && second.is_uppercase()
                    && second.to_ascii_lowercase() == first)
        }
        _ => false,
    }
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

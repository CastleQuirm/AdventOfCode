// Potential improvements:
//

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let mut input_ends = input_lines[0][0].split('-');
    let lower_limit = input_ends
        .next()
        .expect("Didn't read lower limit")
        .parse::<u32>()
        .expect("Couldn't parse");
    let upper_limit = input_ends
        .next()
        .expect("Didn't read upper limit")
        .parse::<u32>()
        .expect("Couldn't parse");
    assert!(input_ends.next().is_none());

    let (answer1, answer2) = dfs(6, Vec::new(), Some(lower_limit), Some(upper_limit));

    (format!("{}", answer1), format!("{}", answer2))
}

fn dfs(
    remaining_digits: u32,
    current_sequence: Vec<u32>,
    min: Option<u32>,
    max: Option<u32>,
) -> (usize, usize) {
    if remaining_digits == 0 {
        let mut found_dupe = false;
        let mut found_just_dupe = false;
        let mut current_digit = current_sequence[0];
        let mut current_repeat = 1;
        let mut comp_ix = 1;
        while comp_ix < 6 {
            if current_sequence[comp_ix] == current_digit {
                current_repeat += 1;
                found_dupe = true;
            } else {
                if current_repeat == 2 {
                    found_just_dupe = true;
                }
                current_repeat = 1;
                current_digit = current_sequence[comp_ix];
            }
            comp_ix += 1;
        }

        if current_repeat == 2 {
            found_just_dupe = true;
        }

        match (found_dupe, found_just_dupe) {
            (true, true) => return (1, 1),
            (true, false) => return (1, 0),
            (false, true) => panic!(),
            (false, false) => return (0, 0),
        }
    }
    let next_digit_min = min.map(|val| {
        val.to_string()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap()
    });
    let next_digit_max = max.map(|val| {
        val.to_string()
            .chars()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap()
    });

    let new_min = min.map(|min| min % 10_u32.pow(remaining_digits - 1));
    let new_max = max.map(|max| max % 10_u32.pow(remaining_digits - 1));

    (next_digit_min
        .unwrap_or(0)
        .max(*current_sequence.last().unwrap_or(&0))..=next_digit_max.unwrap_or(9))
        .map(|next_digit| {
            let mut new_sequence = current_sequence.clone();
            new_sequence.push(next_digit);
            dfs(
                remaining_digits - 1,
                new_sequence,
                if Some(next_digit) == next_digit_min {
                    new_min
                } else {
                    None
                },
                if Some(next_digit) == next_digit_max {
                    new_max
                } else {
                    None
                },
            )
        })
        .fold((0, 0), |(part1_acc, part2_acc), (part1_new, part2_new)| {
            (part1_acc + part1_new, part2_acc + part2_new)
        })
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
            "264793-803935", // INPUT STRING
            "966",           // PART 1 RESULT
            "628",           // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

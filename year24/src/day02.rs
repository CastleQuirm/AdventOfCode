// Potential improvements:
//

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let mut valid_count = 0;
    let mut one_error_count = 0;

    for line in &input_lines[0] {
        // Count the number of directionalities.
        let (mut acc_pos, mut acc_neg, mut acc_eq) = (0, 0, 0);
        let mut numbers = line
            .split(' ')
            .map(|val| val.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        (0..numbers.len() - 1).for_each(|i| match numbers[i].cmp(&numbers[i + 1]) {
            std::cmp::Ordering::Less => acc_pos += 1,
            std::cmp::Ordering::Equal => acc_eq += 1,
            std::cmp::Ordering::Greater => acc_neg += 1,
        });

        // without loss of generality, make the sequence (mostly) ascending.
        if acc_neg > 1 {
            numbers = numbers.into_iter().map(|i| -i).collect::<Vec<_>>();
            // let swap = acc_neg;
            acc_neg = acc_pos;
            // acc_pos = swap;
        }

        match (acc_eq, acc_neg) {
            (0, 0) => {
                if check_sequence(&numbers) {
                    valid_count += 1;
                } else {
                    // Maybe remove just the first, or just the last.
                    if check_sequence(&numbers[1..])
                        || check_sequence(&numbers[0..numbers.len() - 1])
                    {
                        one_error_count += 1;
                    }
                }
            }
            (1, 0) => {
                // remove the duplicate from the sequence. We only remove consecutive dupes and we've checked there's only one.
                numbers.dedup();
                if check_sequence(&numbers) {
                    one_error_count += 1;
                }
            }
            (0, 1) => {
                // find the neg pair. Try removing each
                let dec_index = (0..numbers.len() - 1)
                    .find(|&i| numbers[i] > numbers[i + 1])
                    .unwrap();

                let mut seq = numbers.clone();
                seq.remove(dec_index);
                if check_sequence(&seq) {
                    one_error_count += 1;
                } else {
                    seq = numbers.clone();
                    seq.remove(dec_index + 1);
                    if check_sequence(&seq) {
                        one_error_count += 1;
                    }
                }
            }
            _ => (),
        }
    }

    let answer1 = valid_count;
    // let answer1 = input_lines[0]
    //     .iter()
    //     .filter(|line| {
    //         let mut prev_num = None;
    //         let mut ascending = None;
    //         line.split(' ').all(|val| {
    //             let val = val.parse::<i64>().expect("Couldn't parse");
    //             if let Some(prev) = prev_num {
    //                 let diff: i64 = val - prev;
    //                 if diff.abs() > 3 || diff == 0 {
    //                     return false;
    //                 }
    //                 if let Some(ascending) = ascending {
    //                     if ascending != diff.is_positive() {
    //                         return false;
    //                     }
    //                 } else {
    //                     ascending = Some(diff.is_positive());
    //                 }
    //             }
    //             prev_num = Some(val);
    //             true
    //         })
    //     })
    //     .count();
    let answer2 = valid_count + one_error_count;
    (format!("{}", answer1), format!("{}", answer2))
}

fn check_sequence(seq: &[i64]) -> bool {
    (0..seq.len() - 1).all(|i| {
        let diff = seq[i + 1] - seq[i];
        diff > 0 && diff < 4
    })
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9", // INPUT STRING
            "2", // PART 1 RESULT
            "4", // PART 2 RESULT
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

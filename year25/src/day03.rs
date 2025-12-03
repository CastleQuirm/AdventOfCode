// Potential improvements:
//

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let banks = input_lines[0]
        .iter()
        .map(|line| {
            line.chars()
                .map(|d| d.to_digit(10).expect("bad digit") as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let answer1 = banks.iter().map(|bank| max_from_bank(bank, 2)).sum::<u64>();
    let answer2 = banks
        .iter()
        .map(|bank| max_from_bank(bank, 12))
        .sum::<u64>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn max_from_bank(line: &[u64], final_len: usize) -> u64 {
    // Pick the highest digit from the line, such that there are at least final_len - 1 digits left over.
    // In case of ties, pick the earliest.
    let mut highest_found = 0;
    let mut location = 0;
    for (ix, &battery) in line.iter().enumerate().take(line.len() - final_len + 1) {
        if battery > highest_found {
            {
                highest_found = battery;
                location = ix;
            }
        }
        if highest_found == 9 {
            break;
        }
    }
    if final_len != 1 {
        max_from_bank(&line[location + 1..], final_len - 1)
            + (10u64.pow(final_len as u32 - 1)) * highest_found
    } else {
        highest_found
    }
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "987654321111111
811111111111119
234234234234278
818181911112111", // INPUT STRING
            "357",           // PART 1 RESULT
            "3121910778619", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

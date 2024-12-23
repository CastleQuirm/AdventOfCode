// Potential improvements:
//

use std::collections::{HashMap, VecDeque};

pub fn day22(input_lines: &[Vec<String>]) -> (String, String) {
    let zero_bananas = (0..input_lines[0].len()).map(|_| 0).collect::<Vec<usize>>();
    let mut stock_market: HashMap<VecDeque<isize>, Vec<usize>> = HashMap::new();

    let answer1 = input_lines[0]
        .iter()
        .enumerate()
        .map(|(ix, line)| {
            let mut number = line.parse::<u64>().expect("couldn't parse");
            let mut delta_seq = VecDeque::new();
            (0..2000).for_each(|_| {
                let new_number = compute_next_secret(number);
                let delta = (new_number % 10) as isize - (number % 10) as isize;
                delta_seq.push_back(delta);
                if delta_seq.len() == 5 {
                    delta_seq.pop_front();
                }
                if delta_seq.len() == 4 {
                    let entry = stock_market
                        .entry(delta_seq.clone())
                        .or_insert(zero_bananas.clone());
                    let mut clone_entry = entry.clone();
                    if clone_entry[ix] == 0 {
                        clone_entry[ix] = (new_number % 10) as usize;
                    }
                    *entry = clone_entry;
                }
                number = new_number;
            });
            number
        })
        .sum::<u64>();

    let answer2 = stock_market
        .values()
        .map(|monkeys| monkeys.iter().sum::<usize>())
        .max()
        .expect("No part 2 answer");
    (format!("{}", answer1), format!("{}", answer2))
}

fn compute_next_secret(current_secret: u64) -> u64 {
    ((current_secret ^ ((current_secret * 64) % 16777216))
        ^ ((current_secret ^ ((current_secret * 64) % 16777216)) / 32))
        ^ ((((current_secret ^ ((current_secret * 64) % 16777216))
            ^ ((current_secret ^ ((current_secret * 64) % 16777216)) / 32))
            * 2048)
            % 16777216)
}

#[cfg(test)]
mod tests {
    use super::day22;
    use crate::utils::load_input;

    #[test]
    fn check_day22_case01() {
        full_test(
            "1
10
100
2024", // INPUT STRING
            "37327623", // PART 1 RESULT
            "24",       // PART 2 RESULT
        )
    }

    #[test]
    fn check_day22_case02() {
        full_test(
            "1
2
3
2024", // INPUT STRING
            "37990510", // PART 1 RESULT
            "23",       // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day22(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// Potential improvements:
//

use std::collections::HashMap;

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let mut list_b_frequencies = HashMap::<i64, i64>::new();
    let (mut list_a, mut list_b): (Vec<i64>, Vec<i64>) = input_lines[0]
        .iter()
        .map(|line| {
            let (list_a_str, list_b_str) = line.split_once("   ").expect("Weird line gaps");
            let list_a_num = list_a_str
                .parse::<i64>()
                .expect("Couldn't understand number");
            let list_b_num = list_b_str
                .parse::<i64>()
                .expect("Couldn't understand number");
            *list_b_frequencies.entry(list_b_num).or_default() += 1;
            (list_a_num, list_b_num)
        })
        .unzip();

    list_a.sort();
    list_b.sort();

    let answer1 = (0..list_a.len())
        .map(|i| list_a[i].abs_diff(list_b[i]))
        .sum::<u64>();
    let answer2 = list_a
        .iter()
        .map(|a_val| a_val * list_b_frequencies.get(a_val).unwrap_or(&0))
        .sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "3   4
4   3
2   5
1   3
3   9
3   3", // INPUT STRING
            "11", // PART 1 RESULT
            "31", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day01(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

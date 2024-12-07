use regex::Regex;

// Potential improvements:
//

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let mut answer1 = 0;
    let mut answer2 = 0;
    let line_regex = Regex::new(r"(\d+): (.*)").unwrap();
    input_lines[0].iter().for_each(|line| {
        let cap = line_regex.captures(line).expect("couldn't regex");
        let target = cap[1].parse::<i64>().unwrap();
        let values = cap[2]
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if dfs_values_part1(&target, &values[1..], values[0]) {
            answer1 += target;
            answer2 += target;
        } else if dfs_values_part2(&target, &values[1..], values[0]) {
            answer2 += target;
        }
    });
    (format!("{}", answer1), format!("{}", answer2))
}

fn dfs_values_part1(target: &i64, unused_values: &[i64], current_val: i64) -> bool {
    let try_add = current_val + unused_values[0];
    let try_mul = current_val * unused_values[0];
    if unused_values.len() == 1 {
        try_add == *target || try_mul == *target
    } else {
        dfs_values_part1(target, &unused_values[1..], try_add)
            || dfs_values_part1(target, &unused_values[1..], try_mul)
    }
}

fn dfs_values_part2(target: &i64, unused_values: &[i64], current_val: i64) -> bool {
    let try_add = current_val + unused_values[0];
    let try_mul = current_val * unused_values[0];
    let try_con = current_val * 10_i32.pow(unused_values[0].ilog10() + 1) as i64 + unused_values[0];
    if unused_values.len() == 1 {
        try_add == *target || try_mul == *target || try_con == *target
    } else {
        dfs_values_part2(target, &unused_values[1..], try_add)
            || dfs_values_part2(target, &unused_values[1..], try_mul)
            || dfs_values_part2(target, &unused_values[1..], try_con)
    }
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20", // INPUT STRING
            "3749",  // PART 1 RESULT
            "11387", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

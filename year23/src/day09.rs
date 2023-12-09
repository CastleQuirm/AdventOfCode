// Potential improvements:
//

pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    let extrapolation: (Vec<i64>, Vec<i64>) = input_lines[0]
        .iter()
        .map(|line| extrapolated_numbers(line))
        .unzip();
    let answer1 = extrapolation.0.iter().sum::<i64>();
    let answer2 = extrapolation.1.iter().sum::<i64>();

    (format!("{}", answer1), format!("{}", answer2))
}

fn extrapolated_numbers(line: &str) -> (i64, i64) {
    // Collect the numbers from the line as a Vec
    let sequence = line
        .split_ascii_whitespace()
        .map(|num| num.parse::<i64>().expect("Couldn't parse a number!"))
        .collect::<Vec<i64>>();

    // Calculate the series of derivatives until we have a constant (determined by actually reaching the all-zero line)
    let mut derivatives = Vec::from([sequence]);
    while !derivatives.last().unwrap().iter().all(|&val| val == 0) {
        // Generate another derivative
        derivatives.push(
            derivatives
                .last()
                .unwrap()
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<i64>>(),
        )
    }

    // The next number is just the sum of the last element on each derivative
    let next_num = derivatives
        .iter()
        .map(|given_order| {
            given_order
                .last()
                .expect("We ran out of elements before resolving")
        })
        .sum::<i64>();

    // The previous numer is the sum of the first elements of each derivative but with every other element multiplied by -1
    // We use `chunks_exact()` because EITHER we have an even number of elements (so it's the same as `chunks()`) OR
    // we have an odd number but the last one is all 0s so we can ignore it.
    let prev_num = derivatives
        .chunks_exact(2)
        .map(|deriv_pair| deriv_pair[0][0] - deriv_pair[1][0])
        .sum::<i64>();

    (next_num, prev_num)
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case01() {
        full_test(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45", // INPUT STRING
            "114", // PART 1 RESULT
            "2",   // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day09(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

// Potential improvements:
// 1: Make the format more of a functional programming style
// 2: Improve on the O(n^2) and O(n^3) behaviours.

pub fn day1(input_lines: &[String]) -> (u64, u64) {
    let input_numbers: Vec<u64> = input_lines
        .iter()
        .map(|line| line.parse::<u64>().expect("Line wasn't a number"))
        .collect();
    (calculate2(&input_numbers), calculate3(&input_numbers))
}

fn calculate2(input: &[u64]) -> u64 {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    0
}

fn calculate3(input: &[u64]) -> u64 {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}

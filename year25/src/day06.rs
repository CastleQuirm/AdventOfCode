// Potential improvements:
//

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let input = input_lines[0]
        .iter()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let operation_sequence = input
        .last()
        .unwrap()
        .iter()
        .map(|&operation| match operation {
            "*" => (1, Operation::Multiply),
            "+" => (0, Operation::Add),
            _ => panic!("Bad operation"),
        })
        .collect::<Vec<(u64, Operation)>>();

    let answer1 = operation_sequence
        .iter()
        .enumerate()
        .map(|(ix, &(identity, operation))| {
            input[0..input.len() - 1].iter().fold(identity, |acc, e| {
                let new_val = e[ix].parse::<u64>().expect("bad parse");
                match operation {
                    Operation::Multiply => acc * new_val,
                    Operation::Add => acc + new_val,
                }
            })
        })
        .sum::<u64>();

    let grid_read = input_lines[0][0..input.len() - 1]
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let orig_line_len = grid_read[0].len();
    let mut corrected_grid = vec![vec![' '; grid_read.len()]; orig_line_len];
    grid_read.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, character)| {
            corrected_grid[orig_line_len - x - 1][y] = *character;
        });
    });
    let simple_lines = corrected_grid
        .iter()
        .map(|line| line.iter().collect::<String>().trim().to_owned())
        .collect::<Vec<String>>();
    let mut simple_lines = simple_lines.split(|line| line.is_empty());
    let answer2 = operation_sequence
        .iter()
        .rev()
        .map(|&(identity, operation)| {
            simple_lines
                .next()
                .expect("No numbers left")
                .iter()
                .map(|line| line.parse::<u64>().expect("Bad number read"))
                .fold(identity, |acc, e| match operation {
                    Operation::Multiply => acc * e,
                    Operation::Add => acc + e,
                })
        })
        .sum::<u64>();

    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Copy)]
enum Operation {
    Multiply,
    Add,
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ", // INPUT STRING
            "4277556", // PART 1 RESULT
            "3263827", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day06(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

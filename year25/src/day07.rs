// Potential improvements:
//

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let mut answer1 = 0;
    let map = input_lines[0].clone();
    let mut currently_active = vec![0; map[0].len()];

    let starting_column = map[0].find('S').expect("No starting node");
    currently_active[starting_column] = 1;

    map.iter().for_each(|line| {
        let mut new_active = currently_active.clone();
        line.char_indices().for_each(|(ix, c)| {
            if c == '^' && currently_active[ix] > 0 {
                answer1 += 1;
                if let Some(left_ix) = ix.checked_sub(1) {
                    new_active[left_ix] += currently_active[ix];
                }
                new_active[ix] = 0;
                if ix + 1 < new_active.len() {
                    new_active[ix + 1] += currently_active[ix];
                }
            }
        });
        currently_active = new_active;
    });

    let answer2 = currently_active.iter().sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............", // INPUT STRING
            "21", // PART 1 RESULT
            "40", // PART 2 RESULT
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

// Example simple puzzle to test you've installed correctly.
// This will only be run if you specify to run day 0 specifically.  Running all days will skip this script.

// This (fictional) puzzle input consists of lines containing 2 numbers each.
// For Part 1, we're asked to find the sum of all the numbers in the list.
// For Part 2, we're asked to find the square of the difference between the two numbers in each line,
// then return the sum of those.

// When run with `cargo run 0`, the calling code in main.rs will load the input in the file inputs/0
// and pass that through as the input to the day00() function here as a single &str.
// This function (as with all dayXX templates in this repo) returns two Strings, which will be printed
// out to terminal following the labels "Part 1:" and "Part 2: respectively".

pub fn day00(input_lines: &str) -> (String, String) {
    let answer1 = input_lines.lines().map(sum_numbers_in_line).sum::<i32>();
    let answer2 = input_lines
        .lines()
        .map(square_difference_in_line)
        .sum::<i32>();
    (format!("{}", answer1), format!("{}", answer2))
}

fn sum_numbers_in_line(line: &str) -> i32 {
    line.split(", ")
        .map(|number| number.parse::<i32>().expect("Couldn't parse"))
        .sum::<i32>()
}

fn square_difference_in_line(line: &str) -> i32 {
    let numbers = line
        .split(", ")
        .map(|number| number.parse::<i32>().expect("Couldn't parse"))
        .collect::<Vec<i32>>();
    assert_eq!(numbers.len(), 2);
    (numbers.get(0).unwrap() - numbers.get(1).unwrap()).pow(2)
}

// The template per-day files also come with template UTs.  Most Advent of Code puzzles
// have one or more examples during the puzzle page which can be useful to run your code
// against.

// There are examples here to test just the output for the first or second part of the
// the puzzle, or both together.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day00_part1_case1() {
        assert_eq!(
            day00(
                "1, 2
4, 3"
            )
            .0,
            "10".to_string()
        )
    }

    #[test]
    fn check_day00_part2_case1() {
        assert_eq!(
            day00(
                "1, 2
4, 3"
            )
            .1,
            "2".to_string()
        )
    }

    #[test]
    fn check_day00_both_case1() {
        assert_eq!(
            day00(
                "1, 2
40, 30"
            ),
            ("73".to_string(), "101".to_string())
        )
    }
}

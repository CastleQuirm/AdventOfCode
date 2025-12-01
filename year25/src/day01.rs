// Potential improvements:
//

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut dial = 50;
    input_lines[0].iter().for_each(|line| {
        let direction_mult = match line.chars().next().expect("empty line?") {
            'L' => -1,
            'R' => 1,
            _ => panic!("Unknown direction"),
        };
        let value = line[1..].parse::<i32>().expect("Couldn't parse number");
        answer2 += value / 100;
        // horrible hacks
        if dial == 0 && direction_mult == -1 {
            answer2 -= 1;
        }
        dial += value.rem_euclid(100) * direction_mult;
        if dial.rem_euclid(100) != dial || dial == 0 {
            answer2 += 1;
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            answer1 += 1;
        }
    });

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82", // INPUT STRING
            "3", // PART 1 RESULT
            "6", // PART 2 RESULT
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

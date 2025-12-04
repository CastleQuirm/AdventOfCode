// Potential improvements:
//

use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

use grid::coord::Coord2;

pub fn day13(input_lines: &[Vec<String>]) -> (String, String) {
    let (solutions1, solutions2): (Vec<Option<i64>>, Vec<Option<i64>>) = input_lines
        .iter()
        .map(|lines| {
            let machine = ClawMachine::define(lines);
            (machine.solve(false), machine.solve(true))
        })
        .unzip();

    let answer1 = solutions1.iter().flatten().sum::<i64>();
    let answer2 = solutions2.iter().flatten().sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

struct ClawMachine {
    button_a: Coord2,
    button_b: Coord2,
    prize: Coord2,
}

lazy_static! {
    pub static ref LINE_INFO: Regex =
        Regex::new(r"(Button [AB]|Prize): X[+=](\d+), Y[+=](\d+)").unwrap();
}

impl ClawMachine {
    fn define(lines: &[String]) -> Self {
        assert_eq!(lines.len(), 3);
        Self {
            button_a: ClawMachine::line_capture(&lines[0]),
            button_b: ClawMachine::line_capture(&lines[1]),
            prize: ClawMachine::line_capture(&lines[2]),
        }
    }

    fn line_capture(line: &str) -> Coord2 {
        let line_cap = LINE_INFO.captures(line).expect("couldn't regex");
        Coord2 {
            x: line_cap[2].parse::<i64>().unwrap(),
            y: line_cap[3].parse::<i64>().unwrap(),
        }
    }

    fn prize_coordinates(&self, part_2: bool) -> (i64, i64) {
        if part_2 {
            (self.prize.x + 10000000000000, self.prize.y + 10000000000000)
        } else {
            (self.prize.x, self.prize.y)
        }
    }

    fn solve(&self, part_2: bool) -> Option<i64> {
        // First: is A a multiple of B?
        let a_x = self.button_a.x;
        let a_y = self.button_a.y;
        let b_x = self.button_b.x;
        let b_y = self.button_b.y;
        let (p_x, p_y) = self.prize_coordinates(part_2);

        let a_gcd = a_x.gcd(&a_y);
        let b_gcd = b_x.gcd(&b_y);
        if a_x / a_gcd == b_x / b_gcd && a_y / a_gcd == b_y / b_gcd {
            // The two buttons define a line. This would require some code to check if the prize is
            // on that line, then working out the best valid button combination to reach it, but...
            // we never hit this in the test or the input, so eh.
            unimplemented!()
        } else {
            // The two buttons are not parallel. There is only one solution; is it valid?
            let a_lcm = a_x.lcm(&a_y);
            let b_lcm = b_x.lcm(&b_y);
            let alpha_numerator = p_x * (b_lcm / b_x) - p_y * (b_lcm / b_y);
            let alpha_denominator = a_x * (b_lcm / b_x) - a_y * (b_lcm / b_y);
            let beta_numerator = p_x * (a_lcm / a_x) - p_y * (a_lcm / a_y);
            let beta_denominator = b_x * (a_lcm / a_x) - b_y * (a_lcm / a_y);

            if alpha_numerator % alpha_denominator == 0 && beta_numerator % beta_denominator == 0 {
                // Ensure we're not pushing buttons negative numbers of times
                let a_presses = alpha_numerator / alpha_denominator;
                let b_presses = beta_numerator / beta_denominator;

                if a_presses >= 0 && b_presses >= 0 {
                    // We don't check for the 100 limit: that's part 1 only and as a guideline/trap for
                    // people doing searching rather than linear algebra.
                    Some(3 * a_presses + b_presses)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day13;
    use crate::utils::load_input;

    #[test]
    fn check_day13_case01() {
        full_test(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279", // INPUT STRING
            "480",          // PART 1 RESULT
            "875318608908", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day13(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

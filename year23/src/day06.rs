// Potential improvements:
//

use roots::Roots;

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let times = input_lines[0][0]
        .strip_prefix("Time:")
        .expect("bad prefix time")
        .split_whitespace()
        .map(|time| time.parse::<u32>().expect("couldn't read number"));
    let distances = input_lines[0][1]
        .strip_prefix("Distance:")
        .expect("bad prefix distance")
        .split_whitespace()
        .map(|time| time.parse::<u32>().expect("Couldn't read number"));

    let answer1 = times
        .zip(distances)
        .map(size_of_quadratic_peak)
        .product::<u64>();

    let time = input_lines[0][0]
        .strip_prefix("Time:")
        .expect("Bad prefix time")
        .replace(' ', "")
        .parse::<f64>()
        .expect("Time not a f64");
    let distance_string = input_lines[0][1]
        .strip_prefix("Distance:")
        .expect("Bad prefix distance")
        .replace(' ', "");
    let mut decimal_places = 0;
    let mut factor = 1f64;

    while distance_string[0..distance_string.len() - decimal_places]
        .parse::<f64>()
        .is_err()
    {
        decimal_places += 1;
        factor /= 10f64;
    }

    let remainder = ("0.".to_string() + &distance_string[distance_string.len() - decimal_places..])
        .parse::<f64>()
        .expect("The remainder doesn't work!");
    let distance = distance_string[0..distance_string.len() - decimal_places]
        .parse::<f64>()
        .unwrap()
        + remainder;

    let time = time * factor;
    let answer2 = size_of_quadratic_peak_with_factor(factor, time, distance);

    (format!("{}", answer1), format!("{}", answer2))
}

fn size_of_quadratic_peak((t, d): (u32, u32)) -> u64 {
    // The question equates to finding the integer solutions for the quadratic inequality
    // x^2 - x*t + d > 0

    size_of_quadratic_peak_with_factor(1f64, -1f64 * Into::<f64>::into(t), Into::<f64>::into(d))
}

fn size_of_quadratic_peak_with_factor(f: f64, t: f64, d: f64) -> u64 {
    // The question equates to finding the integer solutions for the quadratic inequality
    // x^2 - x*t + d > 0

    let roots = roots::find_roots_quadratic(f, -1f64 * t, d);
    if let Roots::Two([lower, upper]) = roots {
        let remove_exact_match = if upper.floor() == upper { 1 } else { 0 };
        (upper.floor() - lower.floor()) as u64 - remove_exact_match
    } else {
        panic!("Not got two answers!");
    }
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "Time:      7  15   30
Distance:  9  40  200", // INPUT STRING
            "288",   // PART 1 RESULT
            "71503", // PART 2 RESULT
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

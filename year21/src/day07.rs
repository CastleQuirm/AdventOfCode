// Potential improvements:
// Averages
// - Arguably should 'prove' that the use of median in the first part works reliably
// - Would be nice to work out how to actually use the mean or close enough in part 2.
// - Presumably could use a maths crate rather than implementing myself.
// Code
// - Could commonise the fold code, passing an accumulator function in? (Maintainability; highlights the difference)

pub fn day07(input_lines: &[String]) -> (u64, u64) {
    let mut start_positions = input_lines
        .first()
        .expect("Couldn't read line")
        .split(',')
        .map(|num| num.parse::<i32>().expect("Couldn't parse number"))
        .collect::<Vec<i32>>();
    start_positions.sort_unstable();

    // I'm *fairly* convinced that the best position for part 1 is just the median of the set.
    let median = if start_positions.len() % 2 == 0 {
        let double_median = start_positions[start_positions.len() / 2 - 1]
            + start_positions[start_positions.len() / 2];
        if double_median % 2 != 0 {
            println!("Median got rounded down!");
        }
        double_median / 2
    } else {
        start_positions[(start_positions.len() - 1) / 2]
    };

    let movement_to_median = start_positions
        .iter()
        .fold(0, |dist, pos| dist + (median - pos).abs());

    // I'm vaguely guessing that the best position for part 2 is the arithmetic mean
    // UPDATE: It turns out it was very close... the arithmetic mean of my data came to 466.591;
    // the actual optimal position was 466 rather than 467.  Probably the optimal solution is to
    // calculate this, try rounding in both directions, and pick the better.

    // let sum: i32 = start_positions.iter().sum();
    // let len: i32 = start_positions.len() as i32;
    // let mean: i32 =
    //     if sum % len >= len / 2 {
    //         sum / len + 1
    //     } else {
    //         sum / len
    //     };
    // println!("Sum was {}, mean was {}", sum, mean);
    //
    // let movement_to_mean = start_positions.iter().fold(0, |dist, pos| {
    //     let num_steps = (mean - pos).abs();
    //     dist + num_steps * (num_steps + 1) / 2
    // });
    //
    // (movement_to_median as u64, movement_to_mean as u64)

    let mut best_movement = i32::MAX;

    for test in *start_positions.first().unwrap()..*start_positions.last().unwrap() {
        let movement = start_positions.iter().fold(0, |dist, pos| {
            let num_steps = (test - pos).abs();
            dist + num_steps * (num_steps + 1) / 2
        });
        if movement < best_movement {
            best_movement = movement;
        } else {
            break;
        }
    }

    (movement_to_median as u64, best_movement as u64)
}

#[cfg(test)]
mod tests {
    use super::day07;

    #[test]
    fn check_day07() {
        let input_lines = "16,1,2,0,4,2,7,1,2,14"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day07(&input_lines), (37, 168));
    }
}

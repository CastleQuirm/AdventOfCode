// Potential improvements:
// 1. Would like to do better vector addition when calculating gamma_epsilon_delta.
// 2. A bunch of places (in particular the second part) seems inefficient, although not certain how.
// 3. Probably better binary conversions available, although not certain that's the case when we've got the binary number in a vec.

pub fn day03(input_lines: &[String]) -> (u64, u64) {
    let num_lines = input_lines.len() as u64;
    let line_len = input_lines.first().expect("No input!").len() as u32;
    let collected_output_as_binary = input_lines.iter().map(|s| {
        s.chars()
            .map(|c| c.to_digit(10).expect("Couldn't recognise digit") as u64)
            .collect::<Vec<u64>>()
    });

    let gamma_epsilon_data = collected_output_as_binary
        .clone()
        .reduce(|acc, next| {
            let mut sum = Vec::new();
            if acc.len() != next.len() {
                panic!("Lines weren't all the same length!");
            }
            for i in 0..acc.len() {
                sum.push(acc[i] + next[i]);
            }
            sum
        })
        .expect("Didn't have any elements in the input_lines!");

    let gamma: u64 = gamma_epsilon_data.iter().fold(0, |acc, val| {
        let mut rolling_binary = 2 * acc;
        if 2 * val > num_lines {
            rolling_binary += 1;
        };
        rolling_binary
    });

    let epsilon: u64 = 2_u64.pow(line_len) - gamma - 1;

    let mut oxygen = collected_output_as_binary
        .clone()
        .collect::<Vec<Vec<u64>>>();
    let mut co2 = collected_output_as_binary.collect::<Vec<Vec<u64>>>();
    for i in 0..line_len {
        oxygen = filtration(oxygen, Direction::Most, i as usize);
        co2 = filtration(co2, Direction::Least, i as usize);
    }

    // Could validate that if there's more than one remaining element then they're all equal but that should be
    // a given due to the fact we've filtered to a single value on every position.

    let oxygen = vec_as_binary(
        oxygen
            .first()
            .expect("No number was the most popular?")
            .to_vec(),
    );
    let co2 = vec_as_binary(
        co2.first()
            .expect("No number was the least popular?")
            .to_vec(),
    );

    (gamma * epsilon, oxygen * co2)
}

fn filtration(input: Vec<Vec<u64>>, value: Direction, index: usize) -> Vec<Vec<u64>> {
    let num_inputs = input.len() as u64;

    // Need to just return the input if there's only one element left - this is important when calculating co2!
    if num_inputs == 1 {
        return input;
    }

    let sum_of_chosen_bit: u64 = input
        .iter()
        .map(|l| l[index as usize])
        .collect::<Vec<u64>>()
        .iter()
        .sum();
    let mut chosen_bit: u64 = if 2 * sum_of_chosen_bit < num_inputs {
        0
    } else {
        1
    };

    if value == Direction::Least {
        chosen_bit = 1 - chosen_bit;
    }

    input
        .into_iter()
        .filter(|l| l[index as usize] == chosen_bit)
        .collect::<Vec<Vec<u64>>>()
}

fn vec_as_binary(input: Vec<u64>) -> u64 {
    input.iter().fold(0, |acc, val| 2 * acc + val)
}

#[derive(PartialEq)]
enum Direction {
    Most,
    Least,
}

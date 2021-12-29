use std::collections::HashMap;

pub fn day15(input_lines: &[String]) -> (u64, u64) {
    let mut last_said: HashMap<usize, usize> = HashMap::new();
    let starting_numbers = input_lines[0]
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut next_entry = starting_numbers[0];
    (0..2020 - 1).for_each(|turn_num| {
        next_entry = take_turn(turn_num, &starting_numbers, &mut last_said, next_entry)
    });
    let part1_answer = next_entry as u64;
    (2020 - 1..30000000 - 1).for_each(|turn_num| {
        next_entry = take_turn(turn_num, &starting_numbers, &mut last_said, next_entry)
    });
    let part2_answer = next_entry as u64;
    (part1_answer, part2_answer)
}

fn take_turn(
    turn_num: usize,
    starting_numbers: &[usize],
    last_said: &mut HashMap<usize, usize>,
    next_entry: usize,
) -> usize {
    if turn_num < starting_numbers.len() {
        assert_eq!(starting_numbers[turn_num], next_entry);
        last_said.insert(starting_numbers[turn_num], turn_num);
        if turn_num == starting_numbers.len() - 1 {
            0
        } else {
            starting_numbers[turn_num + 1]
        }
    } else {
        let last_said_val = last_said.insert(next_entry, turn_num);
        match last_said_val {
            Some(previous) => turn_num - previous,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day15;

    #[test]
    fn day15_example() {
        let example = "0,3,6"
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
        assert_eq!(day15(&example), (436, 175594));
    }
}

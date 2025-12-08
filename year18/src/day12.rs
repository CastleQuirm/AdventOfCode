// Potential improvements:
//

use std::collections::HashMap;

pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    // Read the initial state and rules.
    let mut state = PlantRow::new(&input_lines[0][0]);

    let mut rules = ['.'; 32];
    for rule in &input_lines[1] {
        let input_val = treat_as_binary(
            rule.trim_end_matches(['#', '.'])
                .strip_suffix(" => ")
                .expect("Didn't understand rule"),
        );
        let result = rule.chars().last().expect("No last char?");
        rules[input_val] = result;
    }

    (0..20).for_each(|_| state.time(&rules));

    let answer1 = state.value();

    let mut previous_states: HashMap<String, (i64, i64)> = HashMap::new();
    for i in 20_i64..50000000000 {
        state.time(&rules);
        let (time_of_state, leftmost_at_time) = previous_states
            .entry(state.state.clone())
            .or_insert((i, state.leftmost_index));
        if *time_of_state < i {
            let cycle_length = i - *time_of_state;
            let right_shift = state.leftmost_index - *leftmost_at_time;
            // Not 100% certain why I need the -1, but off-by-one errors...!
            let remaining_cycles = (50000000000 - i - 1) / cycle_length;
            let extra_jumps = (50000000000 - i - 1) % cycle_length;
            state = PlantRow {
                state: state.state,
                leftmost_index: state.leftmost_index + right_shift * remaining_cycles,
            };
            (0..extra_jumps).for_each(|_| state.time(&rules));
            break;
        }
    }
    let answer2 = state.value();

    (format!("{}", answer1), format!("{}", answer2))
}

fn treat_as_binary(input: &str) -> usize {
    usize::from_str_radix(&input.replace('#', "1").replace('.', "0"), 2)
        .expect("Didn't get a binary number")
}

#[derive(Debug)]
struct PlantRow {
    state: String,
    leftmost_index: i64,
}

impl PlantRow {
    fn new(input: &str) -> Self {
        Self {
            state: input
                .strip_prefix("initial state: ")
                .expect("Bad first line")
                .to_owned(),
            leftmost_index: 0,
        }
    }

    fn time(&mut self, rules: &[char]) {
        let extended_current_state = ["....", &self.state, "...."].concat();
        // This gives the new state, always extended by 2 characters in either direction.
        let new_state = (2..extended_current_state.len() - 2)
            .map(|i| rules[treat_as_binary(&(&extended_current_state)[i - 2..i + 3])])
            .collect::<String>();
        // Strip any number of '.'s from the start and end - we don't need to track them.
        let new_start = new_state
            .find('#')
            .expect("Assume we won't end up with a completely dead row");
        let new_end = new_state
            .rfind('#')
            .expect("Assume we won't end up with a completely dead row");
        self.state = (&new_state)[new_start..=new_end].to_string();
        self.leftmost_index = self.leftmost_index - 2 + new_start as i64;
    }

    fn value(&self) -> i64 {
        let mut pot_num = self.leftmost_index;
        self.state.chars().fold(0, |mut acc, c| {
            match c {
                '#' => acc += pot_num,
                '.' => (),
                _ => unreachable!(),
            }
            pot_num += 1;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
        full_test(
            "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #", // INPUT STRING
            "325",          // PART 1 RESULT
            "999999999374", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day12(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

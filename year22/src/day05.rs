use itertools::Itertools;
use regex::Regex;

pub fn day05(input_lines: &str) -> (String, String) {
    // Read the first part of the input into a Vec of mutable Vecs
    let mut input = input_lines.lines();

    let mut crate_stacks: Vec<Vec<char>> = Vec::new();

    for line in &mut input {
        assert!(!line.is_empty());
        if line.chars().collect::<Vec<_>>()[1].is_ascii_digit() {
            break;
        }
        // Break the line into chunks.
        let chunks = line.chars().chunks(4);
        for (i, chunk) in chunks.into_iter().enumerate() {
            let letter_or_space = chunk.collect::<Vec<_>>()[1];
            if i >= crate_stacks.len() {
                crate_stacks.push(Vec::new());
            }
            if letter_or_space.is_alphabetic() {
                crate_stacks[i].push(letter_or_space);
            }
        }
    }

    crate_stacks.iter_mut().for_each(|stack| stack.reverse());
    let mut crates_v1 = crate_stacks.clone();
    let mut crates_v2 = crate_stacks;

    // Process the instructions
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").expect("Couldn't build regex");
    for line in input {
        if let Some(instruction) = re.captures(line) {
            let move_count = instruction[1].parse::<usize>().expect("Couldn't parse");
            let from_stack = instruction[2].parse::<usize>().expect("Couldn't parse");
            let to_stack = instruction[3].parse::<usize>().expect("Couldn't parse");

            for _ in 0..move_count {
                let moved_crate = crates_v1[from_stack - 1].pop().expect("No crate to move?");
                crates_v1[to_stack - 1].push(moved_crate);
            }

            let (remaining, new) =
                crates_v2[from_stack - 1].split_at(crates_v2[from_stack - 1].len() - move_count);
            let remaining = remaining.to_vec();
            let new = new.to_vec();
            crates_v2[from_stack - 1] = remaining.to_vec();
            crates_v2[to_stack - 1] = [crates_v2[to_stack - 1].to_vec(), new.to_vec()].concat();
        }
    }

    (
        crates_v1
            .into_iter()
            .map(|mut stack| stack.pop().unwrap_or(' '))
            .collect::<String>(),
        crates_v2
            .into_iter()
            .map(|mut stack| stack.pop().unwrap_or(' '))
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(
            day05(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            ),
            ("CMZ".to_string(), "MCD".to_string())
        )
    }
}

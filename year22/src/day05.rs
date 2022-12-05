use regex::Regex;

pub fn day05(input_lines: &str) -> (String, String) {
    // Read the first part of the input into a Vec of mutable Vecs

    let mut crates: Vec<Vec<char>> = vec![[vec![]; 9]];

    // Process the instructions
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for line in instructions {
        let instruction = re.captures(line).expect("Couldn't regex");
        let move_count = instruction.get(1).expect("Not found").parse::<usize>().expect("Couldn't parse");
        let from_stack = instruction.get(2).expect("Not found").parse::<usize>().expect("Couldn't parse");
        let to_stack = instruction.get(3).expect("Not found").parse::<usize>().expect("Couldn't parse");

        for _ in 0..move_count {
            crates[to_stack].push(crates[from_stack].pop());
        }
    }

    let answer1 = crates.iter().map(|stack| stack.pop().unwrap_or(' ')).collect::<String>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(day05("").0, "0".to_string())
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(day05("").1, "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(day05("    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"), ("CMZ".to_string(), "0".to_string()))
    }
}

// Potential improvements:
//

use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn day16(input_lines: &[Vec<String>]) -> (String, String) {
    // Read the input
    let samples = input_lines[0..(input_lines.len() - 3)]
        .iter()
        .map(|s| Sample::new(s))
        .collect::<Vec<Sample>>();
    let behaves_like = samples
        .iter()
        .map(|s| (s.instruction[0], s.behaves_like()))
        .collect::<Vec<(u64, HashSet<OpCode>)>>();
    let answer1 = behaves_like.iter().filter(|(_, i)| i.len() >= 3).count();

    let mut potential_opcodes = HashMap::new();
    let all_opcodes = HashSet::from([
        OpCode::AddI,
        OpCode::AddR,
        OpCode::MulI,
        OpCode::MulR,
        OpCode::BAnI,
        OpCode::BAnR,
        OpCode::BOrI,
        OpCode::BOrR,
        OpCode::EqIR,
        OpCode::EqRI,
        OpCode::EqRR,
        OpCode::GtIR,
        OpCode::GtRI,
        OpCode::GtRR,
        OpCode::SetI,
        OpCode::SetR,
    ]);
    (0..16_u64).for_each(|i| {
        potential_opcodes.insert(i, all_opcodes.clone());
    });
    // for each behaves_like pair, pull out the actual_opcodes entry and take the intersection (assert non-empty).
    (behaves_like)
        .iter()
        .for_each(|(sample_instruction, sample_options)| {
            let possible_opcodes = potential_opcodes.entry(*sample_instruction).or_default();
            *possible_opcodes = possible_opcodes
                .intersection(sample_options)
                .cloned()
                .collect::<HashSet<OpCode>>();
        });

    (0..16).for_each(|i| {
        let codes = potential_opcodes.get(&i).unwrap();
        println!("Value {} is one of {} options", i, codes.len());
        println!("- {:?}", codes);
    });

    // Filter down the code options.
    let mut confirmed_map: HashMap<u64, OpCode> = HashMap::new();
    while confirmed_map.keys().len() < 16 {
        let solvable_values = (0..16)
            .filter(|i| {
                !confirmed_map.contains_key(i) && potential_opcodes.get(i).unwrap().len() == 1
            })
            .collect::<Vec<u64>>();
        assert_ne!(solvable_values.len(), 0);
        solvable_values.iter().for_each(|i| {
            let solved_opcode = potential_opcodes
                .get(i)
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .clone();
            (0..16).for_each(|j| {
                let possible_opcodes = potential_opcodes.entry(j).or_default();
                possible_opcodes.remove(&solved_opcode);
            });
            let add_solution = confirmed_map.insert(*i, solved_opcode);
            assert!(add_solution.is_none());
        })
    }

    // Run the program
    let mut mem = vec![0; 1000]; // Just assume this many registers is enough
    input_lines.last().unwrap().iter().for_each(|line| {
        let command = Sample::parse_line(r"(\d+) (\d+) (\d+) (\d+)", line);
        let op_code = confirmed_map
            .get(&command[0])
            .expect("Didn't know command?");
        mem[command[3] as usize] = op_code.act(&mem, command[1], command[2]);
    });

    let answer2 = mem[0];
    (format!("{}", answer1), format!("{}", answer2))
}

struct Sample {
    before: Vec<u64>,
    instruction: Vec<u64>,
    after: Vec<u64>,
}

impl Sample {
    fn new(sample: &[String]) -> Self {
        assert_eq!(sample.len(), 3);
        let sample = Self {
            before: Self::parse_line(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]", &sample[0]),
            instruction: Self::parse_line(r"(\d+) (\d+) (\d+) (\d+)", &sample[1]),
            after: Self::parse_line(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]", &sample[2]),
        };
        sample.verify();
        sample
    }

    fn verify(&self) {
        assert_eq!(self.before.len(), 4);
        assert_eq!(self.instruction.len(), 4);
        assert_eq!(self.after.len(), 4);
        assert!(self.instruction[0] <= 15);
        assert!(self.instruction[3] <= 3);
        (0_usize..=3).for_each(|i| {
            if self.instruction[3] != i as u64 {
                assert_eq!(self.before[i], self.after[i])
            }
        });
    }

    fn parse_line(regex: &str, input: &str) -> Vec<u64> {
        let re = Regex::new(regex).unwrap();
        re.captures(input)
            .map(|cap| {
                (1..=4)
                    .map(|i| cap[i].parse::<u64>().expect("Couldn't parse value"))
                    .collect::<Vec<u64>>()
            })
            .unwrap_or_else(|| panic!("Regex didn't match {}", input))
    }

    fn behaves_like(&self) -> HashSet<OpCode> {
        // Naive approach is to just run all 16 possible opcodes and see which come out with the right result.
        let mut possible_opcodes = HashSet::new();
        let target_c = self.after[self.instruction[3] as usize];
        if self.instruction[1] <= 3 {
            let value_a_if_reg = self.before[self.instruction[1] as usize];

            if self.instruction[2] <= 3 {
                let value_b_if_reg = self.before[self.instruction[2] as usize];

                if value_a_if_reg + value_b_if_reg == target_c {
                    possible_opcodes.insert(OpCode::AddR);
                }
                if value_a_if_reg * value_b_if_reg == target_c {
                    possible_opcodes.insert(OpCode::MulR);
                }
                if value_a_if_reg & value_b_if_reg == target_c {
                    possible_opcodes.insert(OpCode::BAnR);
                }
                if value_a_if_reg | value_b_if_reg == target_c {
                    possible_opcodes.insert(OpCode::BOrR);
                }
                if (value_a_if_reg > value_b_if_reg && target_c == 1)
                    || (value_a_if_reg <= value_b_if_reg && target_c == 0)
                {
                    possible_opcodes.insert(OpCode::GtRR);
                }
                if (value_a_if_reg == value_b_if_reg && target_c == 1)
                    || (value_a_if_reg != value_b_if_reg && target_c == 0)
                {
                    possible_opcodes.insert(OpCode::EqRR);
                }
            }

            let value_b_if_imm = self.instruction[2];
            if value_a_if_reg + value_b_if_imm == target_c {
                possible_opcodes.insert(OpCode::AddI);
            }
            if value_a_if_reg * value_b_if_imm == target_c {
                possible_opcodes.insert(OpCode::MulI);
            }
            if value_a_if_reg & value_b_if_imm == target_c {
                possible_opcodes.insert(OpCode::BAnI);
            }
            if value_a_if_reg | value_b_if_imm == target_c {
                possible_opcodes.insert(OpCode::BOrI);
            }
            if (value_a_if_reg > value_b_if_imm && target_c == 1)
                || (value_a_if_reg <= value_b_if_imm && target_c == 0)
            {
                possible_opcodes.insert(OpCode::GtRI);
            }
            if (value_a_if_reg == value_b_if_imm && target_c == 1)
                || (value_a_if_reg != value_b_if_imm && target_c == 0)
            {
                possible_opcodes.insert(OpCode::EqRI);
            }
            if value_a_if_reg == target_c {
                possible_opcodes.insert(OpCode::SetR);
            }
        }
        let value_a_if_imm = self.instruction[1];

        if self.instruction[2] <= 3 {
            let value_b_if_reg = self.before[self.instruction[2] as usize];
            if (value_a_if_imm > value_b_if_reg && target_c == 1)
                || (value_a_if_imm <= value_b_if_reg && target_c == 0)
            {
                possible_opcodes.insert(OpCode::GtIR);
            }
            if (value_a_if_imm == value_b_if_reg && target_c == 1)
                || (value_a_if_imm != value_b_if_reg && target_c == 0)
            {
                possible_opcodes.insert(OpCode::EqIR);
            }
        }

        if value_a_if_imm == target_c {
            possible_opcodes.insert(OpCode::SetI);
        }

        possible_opcodes
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum OpCode {
    AddI,
    AddR,
    MulI,
    MulR,
    BAnI,
    BAnR,
    BOrI,
    BOrR,
    SetI,
    SetR,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl OpCode {
    fn act(&self, mem: &[u64], a: u64, b: u64) -> u64 {
        match self {
            OpCode::AddI => mem[a as usize] + b,
            OpCode::AddR => mem[a as usize] + mem[b as usize],
            OpCode::MulI => mem[a as usize] * b,
            OpCode::MulR => mem[a as usize] * mem[b as usize],
            OpCode::BAnI => mem[a as usize] & b,
            OpCode::BAnR => mem[a as usize] & mem[b as usize],
            OpCode::BOrI => mem[a as usize] | b,
            OpCode::BOrR => mem[a as usize] | mem[b as usize],
            OpCode::SetI => a,
            OpCode::SetR => mem[a as usize],
            OpCode::GtIR => {
                if a > mem[b as usize] {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRI => {
                if mem[a as usize] > b {
                    1
                } else {
                    0
                }
            }
            OpCode::GtRR => {
                if mem[a as usize] > mem[b as usize] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqIR => {
                if a == mem[b as usize] {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRI => {
                if mem[a as usize] == b {
                    1
                } else {
                    0
                }
            }
            OpCode::EqRR => {
                if mem[a as usize] == mem[b as usize] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day16;
    use crate::utils::load_input;

    #[test]
    fn check_day16_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day16(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

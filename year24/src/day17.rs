// Potential improvements:
//

pub fn day17(input_lines: &[Vec<String>]) -> (String, String) {
    let read_num = |s: &str| {
        s.split_ascii_whitespace()
            .last()
            .and_then(|val| val.parse::<u32>().ok())
            .expect("error getting input")
    };
    let mut reg_a = read_num(&input_lines[0][0]) as u64;
    let test1 = reg_a == 729;
    let mut reg_b = read_num(&input_lines[0][1]);
    let mut reg_c = read_num(&input_lines[0][2]);

    let program = input_lines[1][0]
        .split_ascii_whitespace()
        .last()
        .expect("issue!")
        .split(',')
        .map(|val| val.parse::<u32>().expect("Bad number"))
        .collect::<Vec<u32>>();
    let mut program_ptr: usize = 0;

    let mut answer1 = "".to_string();

    while program_ptr < program.len() - 1 {
        let command = program[program_ptr];
        let literal_operand = program[program_ptr + 1];
        let combo_operand = match literal_operand {
            0..=3 => Some(literal_operand as u64),
            4 => Some(reg_a),
            5 => Some(reg_b as u64),
            6 => Some(reg_c as u64),
            _ => None,
        };

        match command {
            0 => reg_a = reg_div(reg_a, combo_operand.unwrap()),
            1 => reg_b ^= literal_operand,
            2 => reg_b = (combo_operand.unwrap() % 8) as u32,
            3 => {
                if reg_a != 0 {
                    program_ptr = literal_operand as usize;
                }
            }
            4 => reg_b ^= reg_c,
            5 => {
                if !answer1.is_empty() {
                    answer1.push(',');
                }
                answer1.push_str(&format!("{}", combo_operand.unwrap() % 8));
            }
            6 => {
                reg_b = TryInto::<u32>::try_into(reg_div(reg_a, combo_operand.unwrap()))
                    .expect("Large B")
            }
            7 => {
                reg_c = TryInto::<u32>::try_into(reg_div(reg_a, combo_operand.unwrap()))
                    .expect("Large C")
            }
            _ => panic!(),
        }
        if command != 3 || reg_a == 0 {
            program_ptr += 2;
        }
    }

    // In my input, and I suspect in all inputs, the strucutre is to repeatedly operate on the last three bits of A, strip them
    // off and repeat until A is 0, with a little complexity added from the larger part of A during the operation. So the
    // approach is to calculate original A in three-bit chunks from top to bottom by iterating backwards through the target program.
    let answer2 = if !test1 {
        let mut answer2_options = Vec::from([0u64]);
        program.iter().rev().for_each(|target_out| {
            answer2_options = answer2_options
                .iter()
                .flat_map(|existing| {
                    (0..8)
                        .filter_map(|val| {
                            let try_a = existing * 8 + val;
                            if calculate_step(&program, try_a, *target_out) {
                                Some(try_a)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<u64>>();
        });
        answer2_options.sort();
        answer2_options[0]
    } else {
        0
    };

    (answer1, format!("{}", answer2))
}

fn reg_div(reg_a: u64, combo_operand: u64) -> u64 {
    reg_a
        / (2u32
            .pow(TryInto::<u32>::try_into(combo_operand).expect("taking 2 to too large a power!"))
            as u64)
}

fn calculate_step(program: &[u32], try_a: u64, target: u32) -> bool {
    let mut reg_a = try_a;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut program_ptr = 0;
    let mut found_target = false;
    while program_ptr < program.len() - 1 {
        let command = program[program_ptr];
        let literal_operand = program[program_ptr + 1];
        let combo_operand = match literal_operand {
            0..=3 => Some(literal_operand as u64),
            4 => Some(reg_a),
            5 => Some(reg_b as u64),
            6 => Some(reg_c as u64),
            _ => None,
        };

        match command {
            0 => reg_a = reg_div(reg_a, combo_operand.unwrap()),
            1 => reg_b ^= literal_operand,
            2 => reg_b = (combo_operand.unwrap() % 8) as u32,
            3 => {
                assert_eq!(program_ptr, program.len() - 2);
                assert_eq!(literal_operand, 0);
                assert!(found_target);
                return true;
            }
            4 => reg_b ^= reg_c,
            5 => {
                if combo_operand.unwrap() % 8 == target.into() {
                    found_target = true;
                } else {
                    return false;
                }
            }
            6 => {
                reg_b = TryInto::<u32>::try_into(reg_div(reg_a, combo_operand.unwrap()))
                    .expect("Large B")
            }
            7 => reg_c = (reg_div(reg_a, combo_operand.unwrap()) % 8) as u32, // Slight hack
            _ => panic!(),
        }
        if command != 3 || reg_a == 0 {
            program_ptr += 2;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::day17;
    use crate::utils::load_input;

    #[test]
    fn check_day17_case01() {
        full_test(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0", // INPUT STRING
            "4,6,3,5,6,3,5,2,1,0", // PART 1 RESULT
            "0",                   // PART 2 RESULT
        )
    }

    #[test]
    fn check_day17_case02() {
        full_test(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0", // INPUT STRING
            "5,7,3,0", // PART 1 RESULT
            "117440",  // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day17(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

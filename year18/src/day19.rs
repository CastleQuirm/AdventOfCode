// Potential improvements:
//

use recap::Regex;

use crate::utils::OpCode;

pub fn day19(input_lines: &[Vec<String>]) -> (String, String) {
    // Read instructions
    let instruction_pointer_ix = &Regex::new(r"#ip (\d)")
        .unwrap()
        .captures(&input_lines[0][0])
        .expect("Didn't match")[1]
        .parse::<usize>()
        .expect("Couldn't parse pointer number");

    let command_regex = Regex::new(r"(\w{4}) (\d+) (\d+) (\d+)").unwrap();

    let program = input_lines[0][1..]
        .iter()
        .enumerate()
        .map(|(_line_no, line)| {
            let parsed = command_regex.captures(line).expect("Didn't match");
            Command {
                // line: line_no,
                instruction: parsed[1]
                    .parse::<OpCode>()
                    .expect("Didn't recognize OpCode"),
                a: parsed[2].parse::<usize>().expect("Bad number"),
                b: parsed[3].parse::<usize>().expect("Bad number"),
                c: parsed[4].parse::<usize>().expect("Bad number"),
            }
        })
        .collect::<Vec<_>>();

    let answer1 = run_program(&program, instruction_pointer_ix, 0);
    let answer2 = run_program(&program, instruction_pointer_ix, 1);
    // let answer2 = 0;

    (format!("{}", answer1), format!("{}", answer2))
}

fn run_program(program: &[Command], instruction_pointer_ix: &usize, reg_0: usize) -> usize {
    let mut registers = [0_usize; 6];
    registers[0] = reg_0;
    let mut instruction_pointer_val = 0;

    let max_program_line = program.len() - 1;

    // Run the code
    while instruction_pointer_val <= max_program_line {
        registers[*instruction_pointer_ix] = instruction_pointer_val;
        let command = &program[instruction_pointer_val];
        registers[command.c] = command
            .instruction
            .act_as_usize(&registers, command.a, command.b);
        instruction_pointer_val = registers[*instruction_pointer_ix];
        instruction_pointer_val += 1;

        if registers[0] >= 2 {
            // We've started the core program...and crucially, know what the third register is.
            break;
        }
    }

    let goal_product = registers[3];
    let mut factor_sum = 0;
    for x in 1..=goal_product {
        if goal_product % x == 0 {
            factor_sum += x + goal_product / x;
        }
        if goal_product / x < x {
            break;
        }
    }
    factor_sum
    // registers[0]
}

struct Command {
    // line: usize,
    instruction: OpCode,
    a: usize,
    b: usize,
    c: usize,
}

#[cfg(test)]
mod tests {
    use super::day19;
    use crate::utils::load_input;

    #[test]
    fn check_day19_case01() {
        full_test(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5", // INPUT STRING
            "6", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day19(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}

//#ip 5
// addi 5 16 5   - 1. Add 16 to the pointer register i.e. skip 16 lines
// seti 1 3 1      - 12-part-1. set reg 1 to 1 [0, 1, 0, 1025, 189, 1]
// seti 1 1 2      - set reg 2 to 1.
// mulr 1 2 4      - set reg 4 to reg1*reg2 [0, 1, 1, 1025, 1, 3]      // [0, 1, 1, X, checker, pointer]
// eqrr 4 3 4      - set reg 4 to 1 if r1*r2==r3 else 0
// addr 4 5 5      - if r1*r2 == r3 skip a line
// addi 5 1 5      - skip next line
// addr 1 0 0      - (r1*r2 == r3) => add r1 on to r0                  // Add r1 to r0 if r1*r2=r3
// addi 2 1 2      - add 1 to r2
// gtrr 2 3 4      - set r4 to 1 if r2 > r3, else 0
// addr 5 4 5      - if r2 > r3, skip a line
// seti 2 4 5      - loop back - next line is the multiply             // Add 1 to r2 and repeat the above until r2 > r3          //
// addi 1 1 1      - add 1 to r1
// gtrr 1 3 4      - set r4 to 1 if r1 > r3, else 0
// addr 4 5 5      - if r1 > r3, skip a line
// seti 1 5 5      -                                                   // ... Overall, this is generating the sum of every factor of r3.
// mulr 5 5 5
// addi 3 2 3    - 2. Add 2 to reg 3.
// mulr 3 3 3    - 3. Square reg 3.
// mulr 5 3 3    - 4. multiply reg 3 by 19
// muli 3 11 3   - 5. multiply reg 3 by 11 [?, 0, 0, 836, 0, 20]
// addi 4 8 4    - 6. add 8 to reg 4
// mulr 4 5 4    - 7. mul reg 4 by 22
// addi 4 13 4   - 8. add 13 to reg 4
// addr 3 4 3    - 9. add reg 4 to reg 3 [?, 0, 0, 1025, 189, 24]
// addr 5 0 5    - 10. jump based on reg 0!!!! <- 0 or 1 based on part
// seti 0 8 5    - 11-part-1.  jump to start (skip first line)
// setr 5 3 4      - 12-part-2. set r4 to 27 [1, 0, 0, 1025, 27, 27]
// mulr 4 5 4      - set r4 to 27*28
// addr 5 4 4      - add 29 to r4
// mulr 5 4 4      - mul r4 by 30
// muli 4 14 4     - mul r4 by 14
// mulr 4 5 4      - mul r4 by 32
// addr 3 4 3      - add r4 to r3
// seti 0 8 0      - set r1 to 0
// seti 0 4 5      - loop to program

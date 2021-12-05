// Possible improvements:
// 1: Some smart way of searching backwards for a solution i.e. find all lines that themselves or swapped will take you to line 642, then all lines that would take you to them, etc.
// 2: Remove the While loop to be replaced by something functional.
// 3: Clean up the swapped instruction code to not have duplicated stuff.
// 4: Improve the general use cases of the functions - is run_program actually too general given it only takes an input in the second case and we don't care about an output it's building in that case?
// 4a: Commonise more subfunctions of run_program (so that we could then call separate parent functions while not duplicating code).

pub fn day8(input_lines: &[String]) -> (u64, u64) {
    let (accumulator_1, possible_switches) = run_program(input_lines, None);
    // println!("Going to run {} times", possible_switches.len());
    let accumulator_2 = possible_switches
        .iter()
        .map(|switch| run_program(input_lines, Some(*switch)).0)
        .find(|&answer| answer != 0)
        .expect("Didn't find an answer for Part 2");
    (accumulator_1, accumulator_2)
}

fn run_program(input_lines: &[String], switch: Option<usize>) -> (u64, Vec<usize>) {
    let mut accumulator = 0;
    let mut pointer = 0;
    let mut possible_switches: Vec<usize> = Vec::new();
    let mut covered_lines = [false; 646]; // hack - I know the input is 646 lines

    while (pointer < input_lines.len()) && !covered_lines[pointer] {
        let separated_instruction = input_lines[pointer].split(' ');
        let instruction = separated_instruction
            .clone()
            .next()
            .expect("No instruction?");
        let amount = separated_instruction
            .clone()
            .nth(1)
            .expect("No counter?")
            .parse::<i32>()
            .expect("Couldn't parse counter");
        covered_lines[pointer] = true;

        match instruction {
            "acc" => {
                accumulator += amount;
                pointer += 1;
            }
            "jmp" => {
                if switch != Some(pointer) {
                    possible_switches.push(pointer);
                    let new_pointer = (pointer as i32) + amount;
                    assert!(new_pointer >= 0);
                    pointer = new_pointer as usize;
                } else {
                    pointer += 1;
                }
            }
            "nop" => {
                if switch != Some(pointer) {
                    possible_switches.push(pointer);
                    pointer += 1;
                } else {
                    let new_pointer = (pointer as i32) + amount;
                    assert!(new_pointer >= 0);
                    pointer = new_pointer as usize;
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    if pointer >= input_lines.len() {
        assert_eq!(pointer, input_lines.len());
    } else if switch.is_some() {
        accumulator = 0;
    }
    // println!("Returning {}", accumulator);
    (accumulator as u64, possible_switches)
}

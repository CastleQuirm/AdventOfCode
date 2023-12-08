// Potential improvements:
//

use crate::opcode::Computer;

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let reset_computer = Computer::from_input(&input_lines[0][0]);
    let answer1 = prep_and_run_computer(&reset_computer, 12, 2);

    let answer2 = (0..100)
        .filter_map(|noun| {
            (0..100)
                .find(|&verb| prep_and_run_computer(&reset_computer, noun, verb) == 19690720)
                .map(|verb| noun * 100 + verb)
        })
        .next()
        .expect("No Part2 Answer found");
    (format!("{}", answer1), format!("{}", answer2))
}

fn prep_and_run_computer(fresh_computer: &Computer, noun: usize, verb: usize) -> usize {
    let mut computer = fresh_computer.clone();
    computer.set_memory_at(1, noun);
    computer.set_memory_at(2, verb);
    computer.run_until_stop();
    computer.get_memory_at(0)
}

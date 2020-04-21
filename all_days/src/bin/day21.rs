//* SCC I think this has to be a manual solution for the springbot code -
// (a) it will be the same for every input, and
// (b) it's that or come up with a learning algorithm!

fn main() {
    let orig_computer = all_days::define_computer("input/day21.txt");

    let mut computer = orig_computer.clone_computer();
    let walk_input = String::from("") +
                        "NOT A J\n" +
                        "NOT B T\n" +
                        "OR T J\n" +
                        "NOT C T\n" +
                        "OR T J\n" +
                        "AND D J\n" +
                        "WALK\n";
    print!("Part 1 Answer: ");
    display_screen(&computer.provide_ascii_input(&walk_input));

    computer = orig_computer.clone_computer();
    let run_input = String::from("") +
                        "NOT A J\n" +
                        "NOT B T\n" +
                        "OR T J\n" +
                        "NOT C T\n" +
                        "OR T J\n" +
                        "AND D J\n" +
                        "OR J T\n" +
                        "AND E T\n" +
                        "OR H T\n" +
                        "AND T J\n" +
                        "RUN\n";
    print!("Part 2 Answer: ");
    display_screen(&computer.provide_ascii_input(&run_input));
}

fn display_screen(output_vec: &Vec<i64>) {
    // let output_string: String = output_vec.iter().filter(|&i| *i <= 255).map(|&i| i as u8 as char).collect();
    // let output_lines: Vec<&str> = output_string.split("\n").collect();

    // for line in output_lines {
    //     println!("{:?}", line);
    // }

    let other_output: Vec<&i64> = output_vec.iter().filter(|&i| *i > 255).collect();
    if !other_output.is_empty() {
        println!("{}", other_output[0]);
    }
}

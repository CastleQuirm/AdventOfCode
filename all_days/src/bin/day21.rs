//* Assumed info:
// - a route can be found with just partitioning the full sequence of following ladders straight to their ends and turning once to the next ladder
//* Other notes:
// - the code for handling simpler paths than I expect (e.g. if the main program should just be "A,A,A,A,A") is probably broken
// - the code in determine_program_inputs() is messy (and C-like) as hell.

use std::slice::Iter;
use self::Direction::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];
        DIRECTIONS.iter()
    }
    pub fn invert(direction: &Direction) -> Direction {
        match direction {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

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
    display_screen(&computer.provide_ascii_input(&run_input));
}

fn display_screen(output_vec: &Vec<i64>) {
    let output_string: String = output_vec.iter().filter(|&i| *i <= 255).map(|&i| i as u8 as char).collect();
    let output_lines: Vec<&str> = output_string.split("\n").collect();

    for line in output_lines {
        println!("{:?}", line);
    }

    let other_output: Vec<&i64> = output_vec.iter().filter(|&i| *i > 255).collect();
    if !other_output.is_empty() {
        println!("Non-ASCII output: {:?}", other_output);
    }
}

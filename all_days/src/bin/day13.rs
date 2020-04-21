// Assumed info: Game can begin with paddle stayin still for three frames.

use std::{thread, time};

fn main() {
    let mut computer = all_days::define_computer("input/day13.txt");
    let mut inputs = vec![0, 0, 0];
    computer.program[0] = 2;
    let mut display: Vec<Vec<usize>> = vec![vec![0; 45]; 24];
    let mut score = 0;
    let mut ball_x_coord = 0;
    let mut ball_direction: i64 = 1;
    // let mut starting_blocks = 0;

    loop {
        thread::sleep(time::Duration::from_millis(5));
        let output_vec = computer.push_input_and_run(&mut inputs);
        if output_vec.len() % 3 !=0 {
            panic!("Non triplet output!");
        }
        if output_vec.is_empty() { break; }

        for i in 0..output_vec.len()/3 {
            let j = (3 * i) as usize;
            if output_vec[j] == -1 {
                if output_vec[j + 1] != 0 {
                    panic!("-1 x co-ord with non-zero y co-ord {}", output_vec[j + 1]);
                }
                score = output_vec[j + 2];
                continue;
            }

            display[output_vec[j + 1] as usize][output_vec[j] as usize] = output_vec[j + 2] as usize;

            if output_vec[j + 2] == 4 {
                // This is the ball!
                ball_direction = if output_vec[j] > ball_x_coord { 1 } else { -1 };
                ball_x_coord = output_vec[j];
            }
        }

        show_display(&display, score);

        inputs = vec![ball_direction];
    }
    // println!("Part 1: {} blocks to begin", starting_blocks);
    println!("Final score: {}", score);
}

fn show_display(display: &[Vec<usize>], score: i64) {
    print!("{}[2J", 27 as char);
    println!("Score: {}", score);
    for line in display {
        let mut display_line: String = String::from("");
        for cell in line {
            let display_char = match cell {
                0 => " ",
                1 => "#",
                2 => "@",
                3 => "-",
                4 => "x",
                _ => panic!("Unknown display character"),
            };
            display_line.push_str(display_char)
        }
        println!("{:?}", display_line);
    }
}

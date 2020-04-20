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
    let mut computer = all_days::define_computer("input/day17.txt");
    computer.program[0] = 2;

    let output_vec = computer.run_computer(&mut Vec::new());
    display_screen(&output_vec);

    let full_seq = parse_screen(&output_vec);

    let (sequence, prog_a, prog_b, prog_c) = determine_program_inputs(&full_seq);
    display_screen(&computer.provide_ascii_input(&sequence));
    display_screen(&computer.provide_ascii_input(&prog_a));
    display_screen(&computer.provide_ascii_input(&prog_b));
    display_screen(&computer.provide_ascii_input(&prog_c));

    let mut last_output = computer.provide_ascii_input("n\n");
    let final_answer = last_output.pop();
    display_screen(&last_output);
    println!("Part 2 answer: {}", final_answer.expect("why no answer?"));
}

fn display_screen(_output_vec: &Vec<i64>) {
    // let output_string: String = output_vec.iter().map(|&i| i as u8 as char).collect();
    // let output_lines: Vec<&str> = output_string.split("\n").collect();

    // for line in output_lines {
    //     println!("{:?}", line);
    // }
}

fn parse_screen(output_vec: &Vec<i64>) -> String {
    // Get grid
    let output_string: String = output_vec.iter().map(|&i| i as u8 as char).collect();
    let output_lines: Vec<Vec<char>> = output_string.split("\n").map(|line| line.chars().collect()).collect();

    // Find droid
    let mut droid_x_pos: i32 = -1;
    let mut droid_y_pos: i32 = -1;
    for i in 0..output_lines.len() {
        let line = &output_lines[i];
        match droid_x(line) {
            Some(index) => {
                droid_x_pos = index as i32;
                droid_y_pos = i as i32;
                break;
            }
            None => continue
        }
    }

    if (droid_x_pos == -1) || (droid_y_pos == -1) {
        panic!("No droid found!");
    }
    let droid_x_pos = droid_x_pos as usize;
    let droid_y_pos = droid_y_pos as usize;

    // Find starting direction and rotation
    let starting_direction =
        droid_direction(&output_lines[droid_y_pos][droid_x_pos])
        .expect("No droid?");
    let mut out_dir = find_out_dir(&output_lines,
                                   (droid_x_pos, droid_y_pos),
                                   None);

    let mut result_string = "".to_string();
    let mut position = (droid_x_pos, droid_y_pos);
    let mut visited_positions: Vec<(usize, usize)> = Vec::new();
    let mut checksum = 0;
    let mut current_dir = starting_direction;
    while out_dir.is_some() {
        result_string.push_str(&add_rotation(current_dir, out_dir.expect("")));
        let mut distance = 0;
        while next_cell_is_strut(&output_lines, position, out_dir.expect("")) {
            distance += 1;
            position = update_position(position, out_dir.expect(""));
            if visited_positions.contains(&position) {
                checksum += position.0 * position.1;
            } else {
                visited_positions.push(position);
            }
        }
        result_string.push_str(&distance.to_string());
        result_string.push_str(&",");
        current_dir = out_dir.expect("");
        out_dir = find_out_dir(&output_lines,
                               position,
                               out_dir.as_ref().map(|dir| Direction::invert(dir)));
    }

    // Calculate intersection answer
    println!("Part 1 answer: {}", checksum);

    // Return string
    result_string[0..result_string.len() - 1].to_string()
}

fn add_rotation(our_dir: Direction, desired_dir: Direction) -> String {
    if (our_dir == North && desired_dir == East) ||
        (our_dir == East && desired_dir == South) ||
        (our_dir == South && desired_dir == West) ||
        (our_dir == West && desired_dir == North) {
            "R,".to_string()
        } else if (our_dir == North && desired_dir == West) ||
        (our_dir == East && desired_dir == North) ||
        (our_dir == South && desired_dir == East) ||
        (our_dir == West && desired_dir == South) {
            "L,".to_string()
        } else {
            panic!("can't turn backwards or stay straight! (current {:?}, desired {:?})", our_dir, desired_dir);
        }
}

fn next_cell_is_strut(grid: &Vec<Vec<char>>,
                      (x_point, y_point): (usize, usize),
                      dir: Direction) -> bool {
    match dir {
        North => y_point != 0 && grid[y_point - 1][x_point] == "#".chars().next().expect(""),
        East => x_point != grid[0].len() - 1 && grid[y_point][x_point + 1] == "#".chars().next().expect(""),
        South => grid[y_point + 1].len() != 0 && grid[y_point + 1][x_point] == "#".chars().next().expect(""),
        West => x_point != 0 && grid[y_point][x_point - 1] == "#".chars().next().expect(""),
    }
}

fn update_position((x_point, y_point): (usize, usize), dir: Direction) -> (usize, usize) {
    match dir {
        North => (x_point, y_point - 1),
        East => (x_point + 1, y_point),
        South => (x_point, y_point + 1),
        West => (x_point - 1, y_point),
    }
}

fn droid_x(line: &Vec<char>) -> Option<usize> {
    line.iter().position(|cell| droid_direction(cell).is_some())
}
fn droid_direction(cell: &char) -> Option<Direction> {
    if *cell == "^".chars().next().expect("") {
        Some(North)
    } else if *cell == "<".chars().next().expect("") {
        Some(West)
    } else if *cell == ">".chars().next().expect("") {
        Some(East)
    } else if *cell == "v".chars().next().expect("") {
        Some(South)
    } else {
        None
    }
}

fn find_out_dir(grid: &Vec<Vec<char>>,
                (x_point, y_point): (usize, usize),
                in_dir: Option<Direction>) -> Option<Direction> {
    let mut out_dir: Option<Direction> = None;
    if y_point != 0 && grid[y_point - 1][x_point] == "#".chars().next().expect("") {
        if in_dir.is_none() || in_dir.expect("") != North {
            out_dir = Some(North);
        }
    }
    if x_point != grid[0].len() - 1 && grid[y_point][x_point + 1] == "#".chars().next().expect("") {
        if in_dir.is_none() || in_dir.expect("") != East {
            out_dir = Some(East);
        }
    }
    if grid[y_point + 1].len() != 0 && grid[y_point + 1][x_point] == "#".chars().next().expect("") {
        if in_dir.is_none() || in_dir.expect("") != South {
            out_dir = Some(South);
        }
    }
    if x_point != 0 && grid[y_point][x_point - 1] == "#".chars().next().expect("") {
        if in_dir.is_none() || in_dir.expect("") != West {
            out_dir = Some(West);
        }
    }

    out_dir
}

fn determine_program_inputs(full_seq: &str) -> (String, String, String, String) {
    let (mut len_a, mut len_b, mut len_c): (usize, usize, usize) = (20, 20, 20);
    let (mut sequence, mut prog_a, mut prog_b, mut prog_c) = ("".to_string(), "".to_string(), "".to_string(), "".to_string());
    let mut string_index;

    'outer: while len_a > 0 {
        // Find a valid program A content
        string_index = 0;
        if &full_seq[string_index + len_a..string_index + len_a + 1] != "," {
            len_a -= 1;
            continue;
        }
        prog_a = full_seq[string_index..string_index + len_a].to_string();
        sequence = "A,".to_string();
        string_index += len_a + 1;

        // Does the next part of the program match A?
        while string_index + len_a <= full_seq.len() &&
              &full_seq[string_index..string_index + len_a] == prog_a {
            sequence.push_str(&String::from("A,"));
            string_index += len_a + 1;
        }

        let end_a_ptr = string_index;
        let end_a_seq = String::from(&sequence);

        while len_b > 0 {
            string_index = end_a_ptr;
            sequence = String::from(&end_a_seq);
            // Find a valid program B content
            if &full_seq[string_index + len_b..string_index + len_b + 1] != "," {
                len_b -= 1;
                continue;
            }
            prog_b = full_seq[string_index..string_index + len_b].to_string();
            sequence.push_str(&"B,".to_string());
            string_index += len_b + 1;

            // Does the next part of the program match A or B?
            while (string_index + len_a <= full_seq.len() &&
                   &full_seq[string_index..string_index + len_a] == prog_a) ||
                  (string_index + len_b <= full_seq.len() &&
                   &full_seq[string_index..string_index + len_b] == prog_b) {

                if string_index + len_a <= full_seq.len() &&
                    &full_seq[string_index..string_index + len_a] == prog_a {
                    sequence.push_str(&String::from("A,"));
                    string_index += len_a + 1;
                } else {
                    sequence.push_str(&String::from("B,"));
                    string_index += len_b + 1;
                }
            }

            let end_b_ptr = string_index;
            let end_b_seq = String::from(&sequence);
            while len_c > 0 {
                string_index = end_b_ptr;
                sequence = String::from(&end_b_seq);
                // Find a valid program C content
                if &full_seq[string_index + len_c..string_index + len_c + 1] != "," {
                    len_c -= 1;
                    continue;
                }
                prog_c = full_seq[string_index..string_index + len_c].to_string();
                sequence.push_str(&"C,".to_string());
                string_index += len_c + 1;

                // Does the next part of the program match A, B or C?
                while (string_index + len_a <= full_seq.len() &&
                       &full_seq[string_index..string_index + len_a] == prog_a) ||
                      (string_index + len_b <= full_seq.len() &&
                       &full_seq[string_index..string_index + len_b] == prog_b) ||
                      (string_index + len_c <= full_seq.len() &&
                       &full_seq[string_index..string_index + len_c] == prog_c) {
                    if string_index + len_a <= full_seq.len() &&
                        &full_seq[string_index..string_index + len_a] == prog_a {
                        sequence.push_str(&String::from("A,"));
                        string_index += len_a + 1;
                    } else if string_index + len_b <= full_seq.len() &&
                        &full_seq[string_index..string_index + len_b] == prog_b {
                        sequence.push_str(&String::from("B,"));
                        string_index += len_b + 1;
                    } else {
                        sequence.push_str(&String::from("C,"));
                        string_index += len_c + 1;
                    }
                }
                if string_index == full_seq.len() + 1 {
                    break 'outer;
                }

                len_c -= 1;
            }
            len_b -= 1;
            len_c = 20;
        }
        len_a -= 1;
        len_b = 20;
    }

    sequence = sequence[0..sequence.len() - 1].to_string();
    if sequence.len() > 20 { panic!("Top level program is too long!");}

    sequence.push_str(&String::from("\n"));
    prog_a.push_str(&String::from("\n"));
    prog_b.push_str(&String::from("\n"));
    prog_c.push_str(&String::from("\n"));

    (sequence.to_string(), prog_a.to_string(), prog_b.to_string(), prog_c.to_string())
}

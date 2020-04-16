use std::fs;

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Clone)]
enum CellColour {
    Unpainted,
    Black,
    White,
}

fn main() {
    let mut computer = define_computer();

    let mut map: Vec<Vec<CellColour>> = vec![vec![CellColour::Black]];
    let mut robot = Robot {
        x_position: 0,
        y_position: 0,
        min_x_position: 0,
        min_y_position: 0,
        x_out_of_bounds: false,
        y_out_of_bounds: false,
        direction: Direction::North,
    };

    let mut painted_cells = 0;

    loop {
        // Get input
        let (unpainted, input) = match map[robot.y_index()][robot.x_index()] {
            CellColour::Unpainted => (true, 0),
            CellColour::Black => (false, 0),
            CellColour::White => (false, 1),
        };
        let mut inputs = vec![input];
        // Run computer, get output.
        let output_vec = computer.run_computer(&mut inputs);
        // If output is empty, terminate
        match output_vec.len() {
            0 => break,
            2 => (),
            _ => panic!("Got an output vec of length {}", output_vec.len()),
        }
        // Update painting count
        if unpainted {
            painted_cells += 1;
        }
        // Paint cell
        map[robot.y_index()][robot.x_index()] = match output_vec[0] {
            0 => CellColour::Black,
            1 => CellColour::White,
            _ => panic!("Unrecognised colour output {}", output_vec[0]),
        };
        // Rotate robot
        robot.rotate(output_vec[1]);
        // Move robot
        robot.move_bot();
        // Update map
        update_map(&mut robot, &mut map);
    }

    println!("Painted {} cells", painted_cells + 1);
    map.reverse();
    for line in map {
        let mut colour_line: String = String::from("");
        for cell in line {
            let colour_char = match cell {
                CellColour::Unpainted => " ",
                CellColour::Black => " ",
                CellColour::White => "#",
            };
            colour_line.push_str(colour_char)
        }
        println!("{:?}", colour_line);
    }
}

fn update_map(robot: &mut Robot, map: &mut Vec<Vec<CellColour>>) {
    // Update outer layer - number of y rows.
    if robot.y_out_of_bounds {
        let new_row: Vec<CellColour> = vec![CellColour::Unpainted; map[0].len()];
        map.insert(0, new_row);
        robot.y_out_of_bounds = false;
    } else if robot.y_index() >= map.len() {
        let new_row: Vec<CellColour> = vec![CellColour::Unpainted; map[0].len()];
        map.push(new_row);
    }
    // Update inner layer - number of x columns
    if robot.x_out_of_bounds {
        for row in map {
            row.insert(0, CellColour::Unpainted);
        }
        robot.x_out_of_bounds = false;
    } else if robot.x_index() >= map[0].len() {
        for row in map {
            row.push(CellColour::Unpainted);
        }
    }
}

struct Robot {
    x_position: i32,
    y_position: i32,
    min_x_position: i32,
    min_y_position: i32,
    x_out_of_bounds: bool,
    y_out_of_bounds: bool,
    direction: Direction,
}
impl Robot {
    fn x_index(&self) -> usize {
        (self.x_position - self.min_x_position) as usize
    }
    fn y_index(&self) -> usize {
        (self.y_position - self.min_y_position) as usize
    }
    fn rotate(&mut self, turn: i64) {
        if turn == 0 {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            };
        } else if turn == 1 {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
            };
        } else {
            panic!("Unknown turn command {}", turn);
        }
    }
    fn move_bot(&mut self) {
        match self.direction {
            Direction::North => self.y_position += 1,
            Direction::West => {
                self.x_position -= 1;
                if self.x_position < self.min_x_position {
                    self.x_out_of_bounds = true;
                    self.min_x_position = self.x_position;
                }
            }
            Direction::South => {
                self.y_position -= 1;
                if self.y_position < self.min_y_position {
                    self.y_out_of_bounds = true;
                    self.min_y_position = self.y_position;
                }
            }
            Direction::East => self.x_position += 1,
        }
    }
}

fn define_computer() -> Computer {
    let string = fs::read_to_string("input_program.txt").expect("Failed to read file");

    let program_iter = string.split(',').map(|i| match i.trim().parse() {
        Ok(num) => num,
        Err(_) => 7,
    });

    Computer {
        program: program_iter.collect(),
        relative_base: 0,
        ptr: 0,
    }
}

struct Computer {
    program: Vec<i64>,
    relative_base: usize,
    ptr: usize,
}
impl Computer {
    fn run_computer(&mut self, inputs: &mut Vec<usize>) -> Vec<i64> {
        let mut instruction = self.program[self.ptr];
        let mut return_pair: Vec<i64> = Vec::new();

        while instruction != 99 {
            match instruction % 100 {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(inputs.pop().expect("Didn't have a parameter")),
                4 => {
                    return_pair.push(self.output());
                    if return_pair.len() == 2 {
                        break;
                    }
                }
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals_to(),
                9 => self.change_relative_base(),
                _ => {
                    panic!("Unknown command {}", instruction);
                }
            }

            instruction = self.program[self.ptr];
        }

        return_pair
    }

    fn add(&mut self) {
        let num_params = 3;
        let indices = self.parameter_indices(num_params);
        self.program[indices[2]] = self.program[indices[0]] + self.program[indices[1]];
        self.ptr += num_params + 1;
    }

    fn multiply(&mut self) {
        let num_params = 3;
        let indices = self.parameter_indices(num_params);
        self.program[indices[2]] = self.program[indices[0]] * self.program[indices[1]];
        self.ptr += num_params + 1;
    }

    fn input(&mut self, input: usize) {
        let num_params = 1;
        let indices = self.parameter_indices(num_params);
        self.program[indices[0]] = input as i64;
        self.ptr += num_params + 1;
    }

    fn output(&mut self) -> i64 {
        let num_params = 1;
        let indices = self.parameter_indices(num_params);
        self.ptr += num_params + 1;
        // println!("{}",self.program[indices[0]]);
        self.program[indices[0]]
    }

    fn jump_if_true(&mut self) {
        let num_params = 2;
        let indices = self.parameter_indices(num_params);
        if self.program[indices[0]] != 0 {
            self.ptr = self.program[indices[1]] as usize;
        } else {
            self.ptr += num_params + 1;
        }
    }

    fn jump_if_false(&mut self) {
        let num_params = 2;
        let indices = self.parameter_indices(num_params);
        if self.program[indices[0]] == 0 {
            self.ptr = self.program[indices[1]] as usize;
        } else {
            self.ptr += num_params + 1;
        }
    }

    fn less_than(&mut self) {
        let num_params = 3;
        let indices = self.parameter_indices(num_params);
        self.program[indices[2]] = if self.program[indices[0]] < self.program[indices[1]] {
            1
        } else {
            0
        };
        self.ptr += num_params + 1;
    }

    fn equals_to(&mut self) {
        let num_params = 3;
        let indices = self.parameter_indices(num_params);
        self.program[indices[2]] = if self.program[indices[0]] == self.program[indices[1]] {
            1
        } else {
            0
        };
        self.ptr += num_params + 1;
    }

    fn change_relative_base(&mut self) {
        let num_params = 1;
        let indices = self.parameter_indices(num_params);
        self.relative_base = (self.relative_base as i64 + self.program[indices[0]]) as usize;
        self.ptr += num_params + 1;
    }

    fn parameter_indices(&mut self, num_params: usize) -> Vec<usize> {
        let instruction = self.program[self.ptr];
        let mut parameter_string = instruction / 100;
        let mut return_indices: Vec<usize> = Vec::new();

        for i in 0..num_params {
            let return_index = match parameter_string % 10 {
                0 => self.program[self.ptr + i + 1] as usize,
                1 => self.ptr + i + 1,
                2 => ((self.relative_base as i64) + self.program[self.ptr + i + 1]) as usize,
                _ => panic!(
                    "Param type was {} for index {}",
                    parameter_string % 10,
                    self.ptr
                ),
            };
            self.append_zeroes(return_index + 1);
            return_indices.push(return_index);
            parameter_string /= 10;
        }

        return_indices
    }

    fn append_zeroes(&mut self, length_needed: usize) {
        if length_needed >= self.program.len() {
            self.program
                .append(&mut vec![0; length_needed - self.program.len()]);
        }
    }
}

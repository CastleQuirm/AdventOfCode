use std::fs;

struct Computer {
    program: Vec<i64>,
    relative_base: usize,
    ptr: usize
}

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
                Direction::East => Direction::North
            };
        } else if turn == 1 {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                Direction::East => Direction::South
            };
        } else { panic!("Unknown turn command {}", turn); }
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
            },
            Direction::South => {
                self.y_position -= 1;
                if self.y_position < self.min_y_position {
                    self.y_out_of_bounds = true;
                    self.min_y_position = self.y_position;
                }
            },
            Direction::East => self.x_position += 1,
        }
    }
}

fn main() {
    let mut computer = define_computer();

    let mut map: Vec<Vec<CellColour>> = vec![vec![CellColour::Unpainted]];
    let mut robot = Robot {
        x_position: 0,
        y_position: 0,
        min_x_position: 0,
        min_y_position: 0,
        x_out_of_bounds: false,
        y_out_of_bounds: false,
        direction: Direction::North
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
        let output_vec = run_computer(&mut computer, &mut inputs);
        // If output is empty, terminate
        match output_vec.len() {
            0 => break,
            2 => (),
            _ => panic!("Got an output vec of length {}", output_vec.len()),
        }
        // Update painting count
        if unpainted { painted_cells += 1; }
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

    println!("Painted {} cells", painted_cells);

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

fn define_computer() -> Computer {
    let string = fs::read_to_string("input_program.txt")
        .expect("Failed to read file");

    let program_iter = string.split(",").map(|i| {
        match i.trim().parse() {
            Ok(num) => num,
            Err(_) => 7,
        }
    });

    Computer {
        program: program_iter.collect(),
        relative_base: 0,
        ptr: 0
    }
}

fn run_computer(computer: &mut Computer, inputs: &mut Vec<usize>) -> Vec<i64> {

    let mut instruction = computer.program[computer.ptr];
    let mut return_pair: Vec<i64> = Vec::new();

    while instruction != 99 {
        match instruction % 100 {
            1 => add(computer),
            2 => multiply(computer),
            3 => input(computer, inputs.pop().expect("Didn't have a parameter")),
            4 => {
                return_pair.push(output(computer));
                if return_pair.len() == 2 { break; }
            },
            5 => jump_if_true(computer),
            6 => jump_if_false(computer),
            7 => less_than(computer),
            8 => equals_to(computer),
            9 => change_relative_base(computer),
            _ => { panic!("Unknown command {}", instruction); }
        }

        instruction = computer.program[computer.ptr];
    }

    return_pair
}

fn add(computer: &mut Computer) {
    let num_params = 3;
    let indices = parameter_indices(computer, num_params);
    computer.program[indices[2]] = computer.program[indices[0]] + computer.program[indices[1]];
    computer.ptr += num_params + 1;
}

fn multiply(computer: &mut Computer) {
    let num_params = 3;
    let indices = parameter_indices(computer, num_params);
    computer.program[indices[2]] = computer.program[indices[0]] * computer.program[indices[1]];
    computer.ptr += num_params + 1;
}

fn input(computer: &mut Computer, input: usize) {
    let num_params = 1;
    let indices = parameter_indices(computer, num_params);
    computer.program[indices[0]] = input as i64;
    computer.ptr += num_params + 1;
}

fn output(computer: &mut Computer) -> i64 {
    let num_params = 1;
    let indices = parameter_indices(computer, num_params);
    computer.ptr += num_params + 1;
    // println!("{}", computer.program[indices[0]]);
    computer.program[indices[0]]
}

fn jump_if_true(computer: &mut Computer) {
    let num_params = 2;
    let indices = parameter_indices(computer, num_params);
    if computer.program[indices[0]] != 0 {
        computer.ptr = computer.program[indices[1]] as usize;
    }
    else {
     computer.ptr += num_params + 1;
    }
}

fn jump_if_false(computer: &mut Computer) {
    let num_params = 2;
    let indices = parameter_indices(computer, num_params);
    if computer.program[indices[0]] == 0 {
        computer.ptr = computer.program[indices[1]] as usize;
    }
    else {
        computer.ptr += num_params + 1;
    }
}

fn less_than(computer: &mut Computer) {
    let num_params = 3;
    let indices = parameter_indices(computer, num_params);
    computer.program[indices[2]] =
        if computer.program[indices[0]] < computer.program[indices[1]] { 1 }
        else { 0 } ;
    computer.ptr += num_params + 1;
}

fn equals_to(computer: &mut Computer) {
    let num_params = 3;
    let indices = parameter_indices(computer, num_params);
    computer.program[indices[2]] =
        if computer.program[indices[0]] == computer.program[indices[1]] { 1 }
        else { 0 } ;
    computer.ptr += num_params + 1;
}

fn change_relative_base(computer: &mut Computer) {
    let num_params = 1;
    let indices = parameter_indices(computer, num_params);
    computer.relative_base = (computer.relative_base as i64 + computer.program[indices[0]]) as usize;
    computer.ptr += num_params + 1;
}

fn parameter_indices(computer: &mut Computer, num_params: usize) -> Vec<usize> {
    let instruction = computer.program[computer.ptr];
    let mut parameter_string = instruction / 100;
    let mut return_indices: Vec<usize> = Vec::new();

    for i in 0..num_params {
        let return_index = match parameter_string % 10 {
            0 => computer.program[computer.ptr + i + 1] as usize,
            1 => computer.ptr + i + 1,
            2 => ((computer.relative_base as i64) + computer.program[computer.ptr + i + 1]) as usize,
            _ => panic!("Param type was {} for index {}", parameter_string % 10, computer.ptr),
        };
        append_zeroes(&mut computer.program, return_index+1);
        return_indices.push(return_index);
        parameter_string /= 10;
    }

    return_indices
}

fn append_zeroes(vector: &mut Vec<i64>, length_needed: usize) {
    if length_needed >= vector.len()
    {
        vector.append(&mut vec![0; length_needed - vector.len()]);
    }
}

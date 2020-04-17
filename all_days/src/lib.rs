use std::fs;

pub fn define_computer(input_file: &str) -> Computer {
    let string = fs::read_to_string(input_file).expect("Failed to read file");

    let program_iter = string.split(',').map(|i| match i.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Couldn't read input program"),
    });

    Computer {
        program: program_iter.collect(),
        relative_base: 0,
        ptr: 0,
    }
}

pub fn run_computer(computer: &mut Computer, inputs: &mut Vec<i64>) -> Vec<i64> {
    let mut instruction = computer.program[computer.ptr];
    let mut return_vec: Vec<i64> = Vec::new();

    while (instruction != 99) && (instruction != 3 || inputs.len() > 0) {
        match instruction % 100 {
            1 => computer.add(),
            2 => computer.multiply(),
            3 => computer.input(inputs.pop().expect("Should have inputs!")),
            4 => return_vec.push(computer.output()),
            5 => computer.jump_if_true(),
            6 => computer.jump_if_false(),
            7 => computer.less_than(),
            8 => computer.equals_to(),
            9 => computer.change_relative_base(),
            _ => {
                panic!("Unknown command {}", instruction);
            }
        }

        instruction = computer.program[computer.ptr];
    }

    return_vec
}

pub struct Computer {
    pub program: Vec<i64>,
    relative_base: usize,
    ptr: usize,
}
impl Computer {
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

    fn input(&mut self, input: i64) {
        let num_params = 1;
        let indices = self.parameter_indices(num_params);
        self.program[indices[0]] = input;
        self.ptr += num_params + 1;
    }

    fn output(&mut self) -> i64 {
        let num_params = 1;
        let indices = self.parameter_indices(num_params);
        self.ptr += num_params + 1;
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

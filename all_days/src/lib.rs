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

pub struct Computer {
    pub program: Vec<i64>,
    relative_base: usize,
    ptr: usize,
}
impl Computer {
    pub fn provide_ascii_input(&mut self, input_string: &str) -> Vec<i64> {
        // println!("{}", input_string);
        let mut input_program: Vec<i64> = input_string.chars().map(|char| char as i64).collect();
        input_program.reverse();
        self.run_computer(&mut input_program)
    }
    pub fn run_computer(&mut self, inputs: &mut Vec<i64>) -> Vec<i64> {
        let mut instruction = self.program[self.ptr] % 100;
        let mut return_vec: Vec<i64> = Vec::new();

        while (instruction != 99) && (instruction != 3 || inputs.len() > 0) {
            match instruction {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(inputs.pop().expect("Should have inputs!")),
                4 => return_vec.push(self.output()),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equals_to(),
                9 => self.change_relative_base(),
                _ => {
                    panic!("Unknown command {}", instruction);
                }
            }

            instruction = self.program[self.ptr] % 100;
        }

        return_vec
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

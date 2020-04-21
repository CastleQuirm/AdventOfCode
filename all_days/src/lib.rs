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
        input_stack: Vec::new(),
    }
}

pub struct Computer {
    pub program: Vec<i64>,
    relative_base: usize,
    ptr: usize,
    input_stack: Vec<i64>,
}
impl Computer {
    pub fn push_to_input(&mut self, new_input: &Vec<i64>) {
        self.input_stack.extend(new_input);
    }
    pub fn clone_computer(&self) -> Computer {
        Computer {
            program: self.program.clone(),
            relative_base: self.relative_base,
            ptr: self.ptr,
            input_stack: self.input_stack.clone(),
        }
    }
    pub fn provide_ascii_input(&mut self, input_string: &str) -> Vec<i64> {
        // println!("{}", input_string);
        let input_program: Vec<i64> = input_string.chars().map(|char| char as i64).collect();
        self.push_to_input(&input_program);
        self.run_computer()
    }
    pub fn push_input_and_run(&mut self, new_input: &Vec<i64>) -> Vec<i64> {
        self.push_to_input(new_input);
        self.run_computer()
    }
    pub fn run_computer(&mut self) -> Vec<i64> {
        let mut instruction = self.program[self.ptr] % 100;
        let mut return_vec: Vec<i64> = Vec::new();

        while (instruction != 99) && (instruction != 3 || self.input_stack.len() > 0) {
            match instruction {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(),
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
    pub fn tick_computer(&mut self) -> Option<i64> {
        let instruction = self.program[self.ptr] % 100;
        if instruction == 3 && self.input_stack.is_empty() {
            self.input_stack.push(-1);
        }

        let mut return_val: Option<i64> = None;

        match instruction {
            1 => self.add(),
            2 => self.multiply(),
            3 => self.input(),
            4 => return_val = Some(self.output()),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals_to(),
            9 => self.change_relative_base(),
            _ => {
                panic!("Unknown command {}", instruction);
            }
        }

        return_val
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

    fn input(&mut self) {
        let num_params = 1;
        let indices = self.parameter_indices(num_params);
        let (input, remaining_input) = self.input_stack.split_first().expect("Should have an input");
        self.program[indices[0]] = *input;
        self.ptr += num_params + 1;
        self.input_stack = remaining_input.to_vec();
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

#[derive(Clone, Debug)]
pub struct Computer {
    mem: Vec<usize>,
    pointer: usize,
}

impl Computer {
    // pub fn from_state(initial_state: &[usize]) -> Self {
    //     Computer {
    //         mem: initial_state.to_vec(),
    //         pointer: 0
    //     }
    // }

    pub fn from_input(initial_line: &str) -> Self {
        Computer {
            mem: initial_line
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            pointer: 0,
        }
    }

    pub fn run_until_stop(&mut self) {
        while self.step() {}
    }

    fn instruction_from_num(&self, instruction_val: usize) -> OpInstruction {
        match instruction_val {
            1 => OpInstruction::Add,
            2 => OpInstruction::Multiply,
            99 => OpInstruction::Stop,
            _ => OpInstruction::Unknown,
        }
    }

    fn step(&mut self) -> bool {
        let operation = self.instruction_from_num(self.mem[self.pointer]);
        match operation {
            OpInstruction::Add | OpInstruction::Multiply => self.operation_combine_two_to_third(
                self.mem[self.pointer + 1],
                self.mem[self.pointer + 2],
                self.mem[self.pointer + 3],
                operation,
            ),
            OpInstruction::Stop => return false,
            OpInstruction::Unknown => unreachable!(),
        }
        true
    }

    fn operation_combine_two_to_third(
        &mut self,
        left_index: usize,
        right_index: usize,
        target_index: usize,
        op: OpInstruction,
    ) {
        match op {
            OpInstruction::Add => {
                self.mem[target_index] = self.mem[left_index] + self.mem[right_index]
            }
            OpInstruction::Multiply => {
                self.mem[target_index] = self.mem[left_index] * self.mem[right_index]
            }
            _ => unreachable!(),
        }
        self.pointer += 4;
    }

    pub fn get_memory_at(&self, index: usize) -> usize {
        self.mem[index]
    }

    pub fn set_memory_at(&mut self, index: usize, value: usize) {
        self.mem[index] = value;
    }
}

enum OpInstruction {
    Add,
    Multiply,
    Stop,
    Unknown,
}

impl OpInstruction {}

#[cfg(test)]
mod tests {
    use super::Computer;

    #[test]
    fn check_day02_add_multiply() {
        let mut computer = Computer::from_input("1,9,10,3,2,3,11,0,99,30,40,50");
        computer.run_until_stop();
        let final_computer = Computer::from_input("3500,9,10,70,2,3,11,0,99,30,40,50");
        assert_eq!(computer.mem, final_computer.mem);
    }
}

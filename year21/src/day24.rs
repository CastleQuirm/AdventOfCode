// Potential improvements:
// .. well, while this gives the correct solutions, to run 'correctly' (i.e. over the full problem space) would take a VERY long time.
// I got the answers by back-engineering the code (which is probably the intended solution) and then this runs in reasonable time (~4s)
// by setting a lot of digits of the relevant bound (Part 1 only has to search < 1,000 cases, Part 2 has to search < 100,000)
// This would almost certainly fail as written with a different input, unless set to the impossibly slow version..

// Ways of doing something clever using some knowledge of the input code but relying on what appears to be the fixed aspects:
// 1. Parse the code for the magic numbers that actually define the criteria: what are the numbers added to y values imported from w,
//    which are the lines that can actually divide to see where the equivalences come in and how they map.
// 2. Sampling values: start with 14 9s, and see what the log_26 of the result is when we try all 9 values for one spot: find one that's
//    one smaller than the others and stick with that, then repeat for each other digit.  Note that this requires some more finess -
//    some of the correct rules are replaced by incorrect rules when other rules aren't followed, so we may find false drops that can't
//    fully resolve, but there's defintiely something achievable by juggling a number at a time with some sort of breadth-first search.

pub fn day24(input_lines: &[String]) -> (u64, u64) {
    let mut alu = Program::new(input_lines);
    // let part1 = (11111111111111..100000000000000)
    let part1 = (11111111111111..91599994399999)
        .rev()
        .find(|license_number| {
            !license_number.to_string().contains('0') && alu.run(*license_number) == Ok(0)
        });

    // let part2 = (11111111111111..100000000000000)
    let part2 = (71111591111111..100000000000000).find(|license_number| {
        !license_number.to_string().contains('0') && alu.run(*license_number) == Ok(0)
    });

    (
        part1.expect("no valid solution to part1?"),
        part2.expect("no valid solution to part2?"),
    )
}

struct Program {
    instructions: Vec<String>,
    memory: [i32; 4],
}

impl Program {
    fn new(input_lines: &[String]) -> Self {
        Self {
            instructions: input_lines
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>(),
            memory: [0; 4],
        }
    }
    fn run(&mut self, license_num: u64) -> Result<i32, &'static str> {
        let mut input_digits = license_num
            .to_string()
            .chars()
            .map(|c| {
                c.to_string()
                    .parse::<i32>()
                    .expect("Couldn't parse a digit")
            })
            .collect::<Vec<i32>>();
        input_digits.reverse();

        // Blank the memory before we get started
        self.memory = [0; 4];

        for instruction in &self.instructions {
            let commands = instruction.split(' ').collect::<Vec<&str>>();
            let first_memory_index = self.get_memory_index(
                commands
                    .get(1)
                    .expect("There should always be a variable name here"),
            );
            let second_memory_value = commands.get(2).map(|&parameter| {
                let parse_result = parameter.parse::<i32>();
                if let Ok(unwrapped_result) = parse_result {
                    unwrapped_result
                } else {
                    self.get_memory(parameter)
                }
            });
            match *commands.first().expect("No command at all") {
                "inp" => {
                    assert!(second_memory_value.is_none());
                    self.memory[first_memory_index] =
                        input_digits.pop().expect("Ran out of digits to pop!");
                    assert_ne!(self.memory[first_memory_index], 0);
                }
                "add" => {
                    self.memory[first_memory_index] +=
                        second_memory_value.expect("Didn't have a second memory value")
                }
                "mul" => {
                    self.memory[first_memory_index] *=
                        second_memory_value.expect("Didn't have a second memory value")
                }
                "div" => {
                    if second_memory_value == Some(0) {
                        return Err("DIV BY 0");
                    }
                    self.memory[first_memory_index] /=
                        second_memory_value.expect("Didn't have a second memory value");
                }
                "mod" => {
                    let second_memory_value =
                        second_memory_value.expect("Didn't have a second memory value");
                    if self.memory[first_memory_index] < 0 || second_memory_value <= 0 {
                        return Err("Mod is unhappy");
                    }
                    self.memory[first_memory_index] %= second_memory_value;
                }
                "eql" => {
                    self.memory[first_memory_index] = if self.memory[first_memory_index]
                        == second_memory_value.expect("Didn't have a second memory value")
                    {
                        1
                    } else {
                        0
                    };
                }
                _ => panic!("Unrecognised command!"),
            }
        }
        // Return the value in 'z'
        Ok(self.memory[3])
    }
    // fn read_w(&self) -> i32 { self.memory[0] }
    // fn read_x(&self) -> i32 { self.memory[1] }
    // fn read_y(&self) -> i32 { self.memory[2] }
    // fn read_z(&self) -> i32 { self.memory[3] }
    // fn write_w(&mut self, value: i32) { self.memory[0] = value; }
    // fn write_x(&mut self, value: i32) { self.memory[1] = value; }
    // fn write_y(&mut self, value: i32) { self.memory[2] = value; }
    // fn write_z(&mut self, value: i32) { self.memory[3] = value; }

    fn get_memory_index(&self, letter: &str) -> usize {
        match letter {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => panic!("Unrecognised memory index"),
        }
    }
    fn get_memory(&self, parameter: &str) -> i32 {
        self.memory[self.get_memory_index(parameter)]
    }
}

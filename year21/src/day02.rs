// Potential improvements:
// Rename SubmarineState fields to be more obvious!
// Make the direction values an enum?
// Work out a way to make things less mutable?  e.g. create a new SubmarineState for each line?

pub fn day02(input_lines: &[String]) -> (u64, u64) {
    let mut submarine = SubmarineState::new();
    input_lines.iter().for_each(|line| { submarine.process_instruction(line) });
    submarine.solution()
}

fn direction_and_val(line: &String) -> (&str, u64) {
    let splitline = line.split(" ").collect::<Vec<&str>>();
    let direction = splitline.first().expect("No first part of splitline");
    let val = splitline.last().expect("No second part of splitline").to_string().parse::<u64>().expect("Couldn't parse the second part as a u64");
    (direction, val)
}
struct SubmarineState {
    // Note that p1_depth is the same as p2_aim would be, so we don't need to track that separately.
    dist: u64,
    p1_depth: u64,
    p2_depth: u64
}

impl SubmarineState {
    fn new() -> Self {
        Self { dist: 0, p1_depth: 0, p2_depth: 0 }
    }
    fn forward(&mut self, val: u64) {
        self.dist += val;
        self.p2_depth += val * self.p1_depth;
    }
    fn up(&mut self, val: u64) {
        self.p1_depth -= val;
    }
    fn down(&mut self, val: u64) {
        self.p1_depth += val;
    }
    fn process_instruction(&mut self, instruction: &String) {
        let (direction, val) = direction_and_val(instruction);
        match direction {
            // Note that p1_depth is the same as p2_aim would be, so we don't need to track that separately.
            "forward" => self.forward(val),
            "up" => self.up(val),
            "down" => self.down(val),
            _ => panic!("Unrecognised direction"),
        }
    }
    fn solution(&self) -> (u64, u64) {
        (self.dist * self.p1_depth, self.dist * self.p2_depth)
    }
}